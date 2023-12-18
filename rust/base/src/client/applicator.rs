use tesseract_one::client::Tesseract;

pub trait Applicator: FnOnce(Tesseract) -> Tesseract {}
impl<F> Applicator for F where F: FnOnce(Tesseract) -> Tesseract {}