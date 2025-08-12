use crate::abi_type::ABIType;
use crate::error::ABIError;
use crate::method::{ABIMethod, ABIMethodArg, ABIMethodArgType};
use serde::{Deserialize, Serialize};
use std::str::FromStr;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Arc56Contract {
    pub name: String,
    pub methods: Vec<Arc56Method>,
    pub source: Option<AppSources>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppSources {
    pub approval: String,
    pub clear: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Arc56Method {
    pub name: String,
    pub desc: Option<String>,
    pub args: Vec<Arc56MethodArg>,
    pub returns: Arc56MethodReturn,
}

impl TryFrom<&Arc56Method> for ABIMethod {
    type Error = ABIError;

    fn try_from(arc56_method: &Arc56Method) -> Result<Self, Self::Error> {
        let args: Result<Vec<ABIMethodArg>, ABIError> =
            arc56_method.args.iter().map(|arg| arg.try_into()).collect();

        let returns = if arc56_method.returns.return_type == "void" {
            None
        } else {
            Some(ABIType::from_str(&arc56_method.returns.return_type)?)
        };

        Ok(ABIMethod::new(
            arc56_method.name.clone(),
            args?,
            returns,
            arc56_method.desc.clone(),
        ))
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Arc56MethodArg {
    pub name: Option<String>,
    #[serde(rename = "type")]
    pub arg_type: String,
    pub desc: Option<String>,
}

impl TryFrom<&Arc56MethodArg> for ABIMethodArg {
    type Error = ABIError;

    fn try_from(arc56_arg: &Arc56MethodArg) -> Result<Self, Self::Error> {
        let arg_type = ABIMethodArgType::from_str(&arc56_arg.arg_type)?;

        Ok(ABIMethodArg::new(
            arg_type,
            arc56_arg.name.clone(),
            arc56_arg.desc.clone(),
        ))
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Arc56MethodReturn {
    #[serde(rename = "type")]
    pub return_type: String,
    pub desc: Option<String>,
}
