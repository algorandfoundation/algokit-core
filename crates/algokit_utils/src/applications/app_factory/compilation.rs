use super::{AppFactory, AppFactoryError};
use crate::applications::app_client::CompilationParams;
use crate::clients::app_manager::{
    CompiledPrograms, DELETABLE_TEMPLATE_NAME, DeploymentMetadata, UPDATABLE_TEMPLATE_NAME,
};
use algokit_abi::arc56_contract::CallOnApplicationComplete;

impl AppFactory {
    pub(crate) fn resolve_compilation_params(
        &self,
        override_cp: Option<CompilationParams>,
    ) -> CompilationParams {
        let mut resolved = override_cp.unwrap_or_default();
        if resolved.deploy_time_params.is_none() {
            resolved.deploy_time_params = self.deploy_time_params.clone();
        }
        if resolved.updatable.is_none() {
            resolved.updatable = self.updatable.or_else(|| {
                self.detect_deploy_time_control_flag(
                    UPDATABLE_TEMPLATE_NAME,
                    CallOnApplicationComplete::UpdateApplication,
                )
            });
        }
        if resolved.deletable.is_none() {
            resolved.deletable = self.deletable.or_else(|| {
                self.detect_deploy_time_control_flag(
                    DELETABLE_TEMPLATE_NAME,
                    CallOnApplicationComplete::DeleteApplication,
                )
            });
        }
        resolved
    }

    pub(crate) async fn compile_programs_with(
        &self,
        override_cp: Option<CompilationParams>,
    ) -> Result<CompiledPrograms, AppFactoryError> {
        let cp = self.resolve_compilation_params(override_cp);
        let (approval_teal, clear_teal) =
            self.app_spec()
                .decoded_teal()
                .map_err(|e| AppFactoryError::CompilationError {
                    message: e.to_string(),
                })?;

        let metadata = DeploymentMetadata {
            updatable: cp.updatable,
            deletable: cp.deletable,
        };

        let approval = self
            .algorand()
            .app()
            .compile_teal_template(
                &approval_teal,
                cp.deploy_time_params.as_ref(),
                Some(&metadata),
            )
            .await
            .map_err(|e| AppFactoryError::CompilationError {
                message: e.to_string(),
            })?;

        let clear = self
            .algorand()
            .app()
            .compile_teal_template(&clear_teal, cp.deploy_time_params.as_ref(), None)
            .await
            .map_err(|e| AppFactoryError::CompilationError {
                message: e.to_string(),
            })?;

        self.update_source_maps(approval.source_map.clone(), clear.source_map.clone());

        Ok(CompiledPrograms { approval, clear })
    }
}
