pub type Result<T> = core::result::Result<T, Error>;
pub type Error = Box<dyn std::error::Error>;

pub mod consts {}

pub mod lm_models {
    pub struct Model<'a> {
        pub name: &'a str,
        pub size: &'a str,
    }

    pub static DEEPSEEK_MODEL: Model = Model {
        name: "deepseek-r1",
        size: "32b",
    };
    pub static LLAMA_2_MODEL: Model = Model {
        name: "llama2",
        size: "latest",
    };
    pub static GEMMA_3_MODEL: Model = Model {
        name: "gemma3",
        size: "12b",
    };

    const OLLAMA_PORT: i32 = 11434;
    const OLLAMA_HOST: &str = "http://localhost";
}
