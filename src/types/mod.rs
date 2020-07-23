pub mod response;
pub mod query;

use edn_rs::{
    Serialize,
};

/// Id to use as reference in Crux, similar to `ids` with `Uuid`. This id is supposed to be a KEYWORD, `Edn::Key`.
#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord)]
pub struct CruxId(String);

impl Serialize for CruxId {
    fn serialize(mut self) -> String {
        self.0.insert(0, ':');
        format!("{}", self.0)
    }
}

impl CruxId {
    pub fn new(id: &str) -> Self {
        Self {0: id.to_string()}
    }
}

// #[macro_export]
// macro_rules! clause {
//     ($v1:tt :$key:tt/$sub:tt $v2:tt) => {{
//         let sv1: String = std::stringify!($v1).into();
//         let skey: String = std::stringify!($key).into();
//         let sv2: String = std::stringify!($v2).into();
//         let ssub: String = std::stringify!($sub).into();

//         let s = format!("[{} :{}/{} {}]", sv1, skey, ssub, sv2);
//         s
//     }};

//     ($v1:tt :$key:tt $v2:tt) => {{
//         let sv1: String = std::stringify!($v1).into();
//         let skey: String = std::stringify!($key).into();
//         let sv2: String = std::stringify!($v2).into();

//         let s = format!("[{} :{} {}]", sv1, skey, sv2);
//         s
//     }};

//     ($v1:tt :$key:tt$(-$name:tt)+ $v2:ty) => {{
//         let sv1: String = std::stringify!($v1).into();
//         let skey: String = std::stringify!($key).into();
//         let sv2: String = std::stringify!($v2).into();
//         let mut named = String::new();
//         $(
//             let n: String = std::stringify!($name).into();
//             named = named + "-" + &n; 
//         )+

//         let s = format!("[{} :{}{} {}]", sv1, skey, named, sv2);
//         s
//     }};

//     ($v1:tt :$key:tt$(-$name:tt)+/$sub:tt $v2:ty) => {{
//         let sv1: String = std::stringify!($v1).into();
//         let skey: String = std::stringify!($key).into();
//         let ssub: String = std::stringify!($sub).into();
//         let sv2: String = std::stringify!($v2).into();
//         let mut named = String::new();
//         $(
//             let n: String = std::stringify!($name).into();
//             named = named + "-" + &n; 
//         )+

//         let s = format!("[{} :{}{}/{} {}]", sv1, skey, named, ssub, sv2);
//         s
//     }};

//     ($v1:tt :$key:tt$(-$name:tt)+/$sub:tt $v2:tt) => {{
//         let sv1: String = std::stringify!($v1).into();
//         let skey: String = std::stringify!($key).into();
//         let ssub: String = std::stringify!($sub).into();
//         let sv2: String = std::stringify!($v2).into();
//         let mut named = String::new();
//         $(
//             let n: String = std::stringify!($name).into();
//             named = named + "-" + &n; 
//         )+

//         let s = format!("[{} :{}{}/{} {}]", sv1, skey, named, ssub, sv2);
//         s
//     }};
// }