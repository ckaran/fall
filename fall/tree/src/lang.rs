use std::cmp::Ordering;
use std::sync::Arc;
use {Text, TextBuf, TextRange, TextEdit, TextEditOp, File, NodeType, NodeTypeInfo, IToken, INode, Metrics, tu};

pub trait LanguageImpl: 'static + Send + Sync {
    fn tokenize<'t>(&'t self, text: Text<'t>) -> Box<Iterator<Item=IToken> + 't>;
    fn parse(&self, text: Text, tokens: &[IToken], metrics: &Metrics) -> INode;
    fn node_type_info(&self, ty: NodeType) -> NodeTypeInfo;
}

#[derive(Clone)]
pub struct Language {
    imp: Arc<LanguageImpl>
}

impl Language {
    pub fn new<I: LanguageImpl>(imp: I) -> Language {
        Language { imp: Arc::new(imp) }
    }

    pub fn parse<T: Into<TextBuf>>(&self, text: T) -> File {
        let text: TextBuf = text.into();
        let metrics = Metrics::new();
        let tokens: Vec<IToken> = metrics.measure_time("lexing", || {
            metrics.record("relexed region", text.len().utf8_len() as u64, "");
            self.imp.tokenize(text.as_slice()).collect()
        });
        let inode = self.imp.parse(text.as_slice(), &tokens, &metrics);
        File::new(self.clone(), text, metrics, tokens, inode)
    }

    pub fn reparse(&self, file: &File, edit: TextEdit) -> File {
        let new_text = edit.apply(file.text());

        let metrics = Metrics::new();
        let tokens = metrics.measure_time("lexing", ||relex(
            |text| self.imp.tokenize(text),
            file.tokens(),
            &edit,
            new_text.as_slice(),
            &metrics
        ));

        let inode = self.imp.parse(new_text.as_slice(), &tokens, &metrics);
        File::new(self.clone(), new_text, metrics, tokens, inode)
    }

    pub fn node_type_info(&self, ty: NodeType) -> NodeTypeInfo {
        self.imp.node_type_info(ty)
    }
}

fn relex<'t, T, I>(
    tokenizer: T,
    old_tokens: &[IToken],
    edit: &TextEdit,
    new_text: Text<'t>,
    metrics: &Metrics
) -> Vec<IToken>
    where T: Fn(Text<'t>) -> I,
          I: Iterator<Item=IToken>
{
    let mut old_tokens = old_tokens.iter().cloned();
    let mut old_len = tu(0);

    let mut new_tokens: Vec<IToken> = Vec::new();
    let mut new_len = tu(0);

    let mut edit_point = tu(0);
    let mut reused = tu(0);

    for op in edit.ops.iter() {
        match *op {
            TextEditOp::Insert(ref buf) => {
                edit_point += buf.len()
            }
            TextEditOp::Copy(range) => {
                let mut ts = tokenizer(new_text.slice(TextRange::from_to(new_len, new_text.len())));
                while new_len < edit_point {
                    let token = ts.next().unwrap();
                    new_len += token.len;
                    new_tokens.push(token)
                }

                while old_len < range.start() {
                    old_len += old_tokens.next().unwrap().len;
                }

                loop {
                    let new_consumed = new_len - edit_point;
                    let old_consumed = old_len - range.start();
                    if new_consumed >= range.len() || old_consumed >= range.len() {
                        break
                    }

                    match new_consumed.cmp(&old_consumed) {
                        Ordering::Less => {
                            let token = ts.next().unwrap();
                            new_len += token.len;
                            new_tokens.push(token)
                        }
                        Ordering::Equal => {
                            for token in &mut old_tokens {
                                old_len += token.len;
                                if old_len >= range.end() {
                                    break;
                                }
                                reused += token.len;
                                new_len += token.len;
                                new_tokens.push(token);
                            }
                        }
                        Ordering::Greater => {
                            let token = old_tokens.next().unwrap();
                            old_len += token.len;
                        }
                    }
                }

                edit_point += range.len()
            }
        }
    }

    let ts = tokenizer(new_text.slice(TextRange::from_to(new_len, new_text.len())));
    new_tokens.extend(ts);
    metrics.record("relexed region", (new_text.len() - reused).utf8_len() as u64, "");
    new_tokens
}
