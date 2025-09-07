/// A simple implementation of the TF-IDF algorithm to extract the most meaning full words
#[derive(Debug, Clone, PartialEq, Default)]
pub struct TfIdf {
    /// List of all unique terms found in the documents
    terms: Vec<String>,
    /// Each inner Vec corresponds to the tf-idf values for a document
    /// The outer Vec corresponds to the list of documents
    tf_idf: Vec<Vec<f32>>,
}

impl TfIdf {
    /// Get the list of all unique terms found in the documents
    pub fn terms(&self) -> Vec<String> {
        self.terms.clone()
    }

    /// Get the tf-idf values for each document
    pub fn tf_idf(&self) -> Vec<Vec<f32>> {
        self.tf_idf.clone()
    }
}

#[derive(Debug, Clone, Default)]
struct Occurence {
    documents: std::collections::HashMap<String, f32>,
}

fn get_file_id(file_index: usize) -> String {
    format!("file-{}", file_index)
}

/// Term Frequency
/// tf(t) = (Number of times term t appears in a document) / (Total number of terms in the document)
fn tf(nb_terms: f32, nb_terms_in_document: f32) -> f32 {
    let tf = nb_terms_in_document / nb_terms;
    tf
}

/// Inverse Document Frequency
/// idf(t) = log_e(Total number of documents / Number of documents with term t in it) + 1
fn idf(nb_of_documents: f32, nb_docs_with_term: f32) -> f32 {
    let idf = (nb_of_documents / (1.0 + nb_docs_with_term)).ln() + 1.0;
    idf
}

//FIXME: slow
/// compute the term frequency and inverse document frequency ratio to
/// determine the most meaning full word found in the `content`.
pub fn tf_idf(contents: Vec<String>) -> TfIdf {
    let mut occurences: std::collections::HashMap<String, Occurence> =
        std::collections::HashMap::new();

    for (i, content) in contents.iter().enumerate() {
        for word in content
            .to_lowercase()
            .split_whitespace()
            .filter(|w| !w.is_empty() && w.len() > 2)
        {
            let entry = occurences.entry(word.to_string()).or_default();
            let count = entry.documents.entry(get_file_id(i)).or_default();
            *count += 1.0;
        }
    }
    let mut tf_idf = vec![];
    let mut all_words = occurences.keys().cloned().collect::<Vec<String>>();
    all_words.sort();

    let nb_of_documents = contents.len() as f32;
    for i in 0..contents.len() {
        let mut v = vec![];

        let file_id = &get_file_id(i);
        let nb_terms: f32 = contents[i].to_lowercase().split_whitespace().count() as f32;
        for word in &all_words {
            let occurence = occurences.get(word).unwrap();
            let nb_terms_in_document = occurence.documents.get(file_id).cloned().unwrap_or(0.0);
            let nb_docs_with_term = occurence.documents.len() as f32;

            let tf = tf(nb_terms, nb_terms_in_document);
            let idf = idf(nb_of_documents, nb_docs_with_term);
            v.push(tf * idf);
        }

        tf_idf.push(v);
    }

    TfIdf {
        terms: all_words,
        tf_idf,
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn should_return_empty_array_for_empty_array() {
        assert_eq!(
            tf_idf(vec![]),
            TfIdf {
                terms: vec![],
                tf_idf: vec![]
            }
        );
    }

    #[test]
    fn should_use_the_inputed_weights_to_change_the_content() {
        let content = vec![
            "test example example".to_string(),
            "test after test could".to_string(),
        ];
        let result = tf_idf(content);
        assert_eq!(
            result,
            TfIdf {
                terms: vec![
                    "after".to_string(),
                    "could".to_string(),
                    "example".to_string(),
                    "test".to_string(),
                ],
                tf_idf: vec![
                    vec![0.0, 0.0, 0.6666667, 0.19817832],
                    vec![0.25, 0.25, 0.0, 0.29726747],
                ]
            }
        );
    }

    #[test]
    fn should_consider_words_with_less_than_3_characters() {
        let content = vec![
            "an example example".to_string(),
            "it after it could".to_string(),
        ];
        let result = tf_idf(content);
        assert_eq!(
            result,
            TfIdf {
                terms: vec![
                    "after".to_string(),
                    "could".to_string(),
                    "example".to_string(),
                ],
                tf_idf: vec![vec![0.0, 0.0, 0.6666667], vec![0.25, 0.25, 0.0],]
            }
        );
    }
}
