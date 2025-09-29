use algokit_abi::CallOnApplicationComplete;

use super::{AppFactory, AppFactoryError};
use crate::applications::app_client::CompilationParams;
use crate::clients::app_manager::{
    CompiledPrograms, DELETABLE_TEMPLATE_NAME, DeploymentMetadata, UPDATABLE_TEMPLATE_NAME,
};

impl AppFactory {
    pub(crate) fn resolve_compilation_params(
        &self,
        compilation_params: Option<CompilationParams>,
    ) -> CompilationParams {
        let mut resolved = compilation_params.unwrap_or_default();

        // Merge factory params if available
        if let Some(factory_params) = &self.compilation_params {
            resolved.deploy_time_params = resolved
                .deploy_time_params
                .or_else(|| factory_params.deploy_time_params.clone());
            resolved.updatable = resolved.updatable.or(factory_params.updatable);
            resolved.deletable = resolved.deletable.or(factory_params.deletable);
        }

        // Auto-detect flags from spec if still unset
        resolved.updatable = resolved.updatable.or_else(|| {
            self.detect_deploy_time_control_flag(
                UPDATABLE_TEMPLATE_NAME,
                CallOnApplicationComplete::UpdateApplication,
            )
        });
        resolved.deletable = resolved.deletable.or_else(|| {
            self.detect_deploy_time_control_flag(
                DELETABLE_TEMPLATE_NAME,
                CallOnApplicationComplete::DeleteApplication,
            )
        });

        resolved
    }

    pub(crate) async fn compile(
        &self,
        compilation_params: Option<CompilationParams>,
    ) -> Result<CompiledPrograms, AppFactoryError> {
        let compilation_params = self.resolve_compilation_params(compilation_params);

        let (approval_teal, clear_teal) =
            self.app_spec()
                .decoded_teal()
                .map_err(|e| AppFactoryError::CompilationError {
                    message: e.to_string(),
                })?;

        let metadata = DeploymentMetadata {
            updatable: compilation_params.updatable,
            deletable: compilation_params.deletable,
        };

        let approval = self
            .algorand()
            .app()
            .compile_teal_template(
                &approval_teal,
                compilation_params.deploy_time_params.as_ref(),
                Some(&metadata),
            )
            .await
            .map_err(|e| AppFactoryError::CompilationError {
                message: e.to_string(),
            })?;

        let clear = self
            .algorand()
            .app()
            .compile_teal_template(
                &clear_teal,
                compilation_params.deploy_time_params.as_ref(),
                None,
            )
            .await
            .map_err(|e| AppFactoryError::CompilationError {
                message: e.to_string(),
            })?;

        self.update_source_maps(approval.source_map.clone(), clear.source_map.clone());

        Ok(CompiledPrograms { approval, clear })
    }
}
