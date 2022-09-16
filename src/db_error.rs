use std::fmt;

enum DBErrorType {
    Request,
	NotFound,
	ParseText
}

pub struct DBError {
    kind: DBErrorType,
    message: String,
}

impl DBError {
	pub fn not_found_error() -> DBError {
		DBError { kind: DBErrorType::NotFound, message: String::from("Not Found")}
	}
	pub fn parse_text_error() -> DBError {
		DBError { kind: DBErrorType::ParseText, message: String::from("Unable to parse text")}
	}
}

impl From<reqwest::Error> for DBError {
    fn from(error: reqwest::Error) -> Self {
        DBError {
            kind: DBErrorType::Request,
            message: error.to_string(),
        }
    }
}

impl fmt::Display for DBError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let err_msg = match &self.kind {
            DBErrorType::Request => {
                format!("Error when making request")
            }
            DBErrorType::NotFound => {
                format!("Not Found")
            }
            DBErrorType::ParseText => {
                format!("Unable to parse text")
            }
        };

        write!(f, "{}", err_msg)
    }
}

impl std::fmt::Debug for DBError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let err_msg = match &self.kind {
            DBErrorType::Request => {
                format!("Error when making request => {:#?}", self.message)
            }
			DBErrorType::NotFound => {
				String::from("Not Found")
            }
			DBErrorType::ParseText => {
				String::from("Unable to parse text")
            }
        };

        write!(f, "{}", err_msg)
    }
}