use solana_program_test::*;

mod defaultrenderer_test {
    use anchor_lang::{InstructionData, ToAccountMetas};
    use libreplex_default_renderer::accounts::RenderContext;
    use libreplex_default_renderer::instruction::Canonical;
    use libreplex_default_renderer::instructions::RenderInput;
    use solana_program::{instruction::Instruction, pubkey::Pubkey};
    use solana_sdk::{signature::Keypair, signer::Signer, transaction::Transaction};

    use super::*;
    #[tokio::test]
    async fn create_creator() {
        let program = ProgramTest::new(
            "default_renderer",
            libreplex_default_renderer::ID,
            processor!(libreplex_default_renderer::entry),
        );

        let mut context = program.start_with_context().await;
        let authority = context.payer.pubkey();

        let metadata = Keypair::new();
        let metadata_extension = Keypair::new();
        let group = Keypair::new();

        let render_canonical = RenderContext {
            metadata: metadata.pubkey(),
            metadata_extension: metadata_extension.pubkey(),
            group: group.pubkey(),
        };

        let render_input = Canonical {
            render_input: RenderInput {},
        };

        let render_ix = Instruction {
            data: render_input.data(),
            program_id: libreplex_default_renderer::ID,
            accounts: render_canonical.to_account_metas(None),
        };

        let render_tx = Transaction::new_signed_with_payer(
            &[render_ix],
            Some(&authority),
            &[&context.payer],
            context.last_blockhash,
        );

        // let signature = render_tx.signatures[0];

        // context
        //     .banks_client
        //     .process_transaction(render_tx)
        //     .await
        //     .unwrap();

        // let result = context
        //     .banks_client
        //     .get_transaction_status(signature).await.unwrap();

        let result = context
            .banks_client
            .simulate_transaction(render_tx)
            .await
            .unwrap();

        let result_data = result.simulation_details.unwrap().return_data.unwrap().data;
        println!("RESULT: {:?}", result_data);
        // std::fs::write("c.bmp", result_data).unwrap();
        // msg!("{}", result)
    }
}
