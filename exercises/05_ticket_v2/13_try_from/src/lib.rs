// TODO: Implement `TryFrom<String>` and `TryFrom<&str>` for `Status`.
//  The parsing should be case-insensitive.

use std::fmt::Error;

#[derive(Debug, PartialEq, Clone)]
enum Status {
    ToDo,
    InProgress,
    Done,
    Error,
}

impl TryFrom<String> for Status {
    fn try_from(value: String) -> Result<Self, Self::Error> {
	if value.tolowercase() == "todo" {
	    return Ok(Status::ToDo);
	}
	else if value.to_lowercase() == "inprogress" {
	    return Ok(Status::InProgress);
	}
	else if value.to_lowercase() == "done" {
	    return Ok(Status::Done);
	}
	else if value.to_lowercase() == "error" {
	    return Ok(Status::Error);	    
	}

	return Error(Self::Error)
    }
}

impl TryFrom<&str> for Status {
    fn try_from(value:&str) -> Result<Self, Self::Error> {
	if value == "ToDO" {
	    return Ok(Status::ToDo);
	}
	else if value == "inproGress" {
	    return Ok(Status::InProgress);
	}
	else if value == "Done" {
	    return Ok(Status::Done);
	}
	else if value == "Error" {
	    return Ok(Status::Error);	    
	}
	return Error(Status::Error)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::convert::TryFrom;

    #[test]
    fn test_try_from_string() {
        let status = Status::try_from("ToDO".to_string()).unwrap();
        assert_eq!(status, Status::ToDo);

        let status = Status::try_from("inproGress".to_string()).unwrap();
        assert_eq!(status, Status::InProgress);

        let status = Status::try_from("Done".to_string()).unwrap();
        assert_eq!(status, Status::Done);
    }

    #[test]
    fn test_try_from_str() {
        let status = Status::try_from("todo").unwrap();
        assert_eq!(status, Status::ToDo);

        let status = Status::try_from("inprogress").unwrap();
        assert_eq!(status, Status::InProgress);

        let status = Status::try_from("done").unwrap();
        assert_eq!(status, Status::Done);
    }
}
