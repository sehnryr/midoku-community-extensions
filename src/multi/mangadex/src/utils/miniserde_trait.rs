use miniserde::json;

pub trait BorrowType {
    fn borrow_bool(&self) -> Result<&bool, ()>;
    fn borrow_number(&self) -> Result<&json::Number, ()>;
    fn borrow_string(&self) -> Result<&String, ()>;
    fn borrow_array(&self) -> Result<&json::Array, ()>;
    fn borrow_object(&self) -> Result<&json::Object, ()>;
}

impl BorrowType for json::Value {
    fn borrow_bool(&self) -> Result<&bool, ()> {
        match self {
            json::Value::Bool(b) => Ok(b),
            _ => Err(()),
        }
    }

    fn borrow_number(&self) -> Result<&json::Number, ()> {
        match self {
            json::Value::Number(n) => Ok(n),
            _ => Err(()),
        }
    }

    fn borrow_string(&self) -> Result<&String, ()> {
        match self {
            json::Value::String(s) => Ok(s),
            _ => Err(()),
        }
    }

    fn borrow_array(&self) -> Result<&json::Array, ()> {
        match self {
            json::Value::Array(a) => Ok(a),
            _ => Err(()),
        }
    }

    fn borrow_object(&self) -> Result<&json::Object, ()> {
        match self {
            json::Value::Object(o) => Ok(o),
            _ => Err(()),
        }
    }
}

pub trait TakeType {
    fn take_bool(&self) -> Result<bool, ()>;
    fn take_number(self) -> Result<json::Number, ()>;
    fn take_string(&self) -> Result<String, ()>;
    fn take_array(self) -> Result<json::Array, ()>;
    fn take_object(self) -> Result<json::Object, ()>;
}

impl TakeType for json::Value {
    fn take_bool(&self) -> Result<bool, ()> {
        match self {
            json::Value::Bool(b) => Ok(b.clone()),
            _ => Err(()),
        }
    }

    fn take_number(self) -> Result<json::Number, ()> {
        match self {
            json::Value::Number(n) => Ok(n),
            _ => Err(()),
        }
    }

    fn take_string(&self) -> Result<String, ()> {
        match self {
            json::Value::String(s) => Ok(s.clone()),
            _ => Err(()),
        }
    }

    fn take_array(self) -> Result<json::Array, ()> {
        match self {
            json::Value::Array(a) => Ok(a),
            _ => Err(()),
        }
    }

    fn take_object(self) -> Result<json::Object, ()> {
        match self {
            json::Value::Object(o) => Ok(o),
            _ => Err(()),
        }
    }
}

pub trait GetType {
    fn get_bool<S: AsRef<str>>(&self, key: S) -> Result<&bool, ()>;
    fn get_number<S: AsRef<str>>(&self, key: S) -> Result<&json::Number, ()>;
    fn get_string<S: AsRef<str>>(&self, key: S) -> Result<&String, ()>;
    fn get_array<S: AsRef<str>>(&self, key: S) -> Result<&json::Array, ()>;
    fn get_object<S: AsRef<str>>(&self, key: S) -> Result<&json::Object, ()>;
}

impl GetType for json::Object {
    fn get_bool<S: AsRef<str>>(&self, key: S) -> Result<&bool, ()> {
        self.get(key.as_ref())
            .ok_or(())
            .and_then(|v| v.borrow_bool())
    }

    fn get_number<S: AsRef<str>>(&self, key: S) -> Result<&json::Number, ()> {
        self.get(key.as_ref())
            .ok_or(())
            .and_then(|v| v.borrow_number())
    }

    fn get_string<S: AsRef<str>>(&self, key: S) -> Result<&String, ()> {
        self.get(key.as_ref())
            .ok_or(())
            .and_then(|v| v.borrow_string())
    }

    fn get_array<S: AsRef<str>>(&self, key: S) -> Result<&json::Array, ()> {
        self.get(key.as_ref())
            .ok_or(())
            .and_then(|v| v.borrow_array())
    }

    fn get_object<S: AsRef<str>>(&self, key: S) -> Result<&json::Object, ()> {
        self.get(key.as_ref())
            .ok_or(())
            .and_then(|v| v.borrow_object())
    }
}
