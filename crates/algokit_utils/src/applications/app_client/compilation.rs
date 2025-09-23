use super::{AppClient, AppClientError};
use crate::{
    Config, EventType,
    applications::app_client::types::CompilationParams,
    clients::app_manager::DeploymentMetadata,
    config::{AppCompiledEventData, EventData},
};

use crate::clients::app_manager::{CompiledPrograms, CompiledTeal};

impl AppClient {
    /// Compile the application's approval and clear programs with optional template parameters.
    pub async fn compile(
        &self,
        compilation_params: &CompilationParams,
    ) -> Result<CompiledPrograms, AppClientError> {
        let approval = self.compile_approval(compilation_params).await?;
        let clear = self.compile_clear(compilation_params).await?;

        // Emit AppCompiled event when debug flag is enabled
        if Config::debug() {
            let app_name = self.app_name.clone();
            let approval_map = approval.source_map.clone();
            let clear_map = clear.source_map.clone();

            let event = AppCompiledEventData {
                app_name,
                approval_source_map: approval_map,
                clear_source_map: clear_map,
            };
            Config::events()
                .emit(EventType::AppCompiled, EventData::AppCompiled(event))
                .await;
        }

        Ok(CompiledPrograms { approval, clear })
    }

    async fn compile_approval(
        &self,
        compilation_params: &CompilationParams,
    ) -> Result<CompiledTeal, AppClientError> {
        // 1) Decode TEAL from ARC-56 source
        let (teal, _) =
            self.app_spec
                .decoded_teal()
                .map_err(|e| AppClientError::CompilationError {
                    message: e.to_string(),
                })?;

        // 2-4) Compile via AppManager helper with template params and deploy-time controls
        let metadata = DeploymentMetadata {
            updatable: compilation_params.updatable,
            deletable: compilation_params.deletable,
        };

        let compiled = self
            .algorand()
            .app()
            .compile_teal_template(
                &teal,
                compilation_params.deploy_time_params.as_ref(),
                Some(&metadata),
            )
            .await
            .map_err(|e| AppClientError::AppManagerError { source: e })?;

        Ok(compiled)
    }

    async fn compile_clear(
        &self,
        compilation_params: &CompilationParams,
    ) -> Result<CompiledTeal, AppClientError> {
        // 1) Decode TEAL from ARC-56 source
        let (_, teal) =
            self.app_spec
                .decoded_teal()
                .map_err(|e| AppClientError::CompilationError {
                    message: e.to_string(),
                })?;

        // 2-4) Compile via AppManager helper with template params; no deploy-time controls for clear
        let compiled = self
            .algorand()
            .app()
            .compile_teal_template(&teal, compilation_params.deploy_time_params.as_ref(), None)
            .await
            .map_err(|e| AppClientError::AppManagerError { source: e })?;

        Ok(compiled)
    }
}
