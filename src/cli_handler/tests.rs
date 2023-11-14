mod parser_tests {
    use crate::cli_handler::parser::{parse_cli, ArgValue};

    #[test]
    #[should_panic(expected = "[Cli Parser] Expect to receive non-empty command input.")]
    fn test_parser_no_command_panic() {
        parse_cli(vec![], vec![], String::from(""));
    }

    #[test]
    #[should_panic]
    fn test_parser_unexpected_arg_panic() {
        parse_cli(
            vec![String::from("my-cmd"), String::from("unexpected-arg")],
            vec![String::from("expected-arg")],
            String::from(""),
        );
    }

    #[test]
    #[should_panic]
    fn test_parser_repeating_argvalue_no_value_panic() {
        parse_cli(
            vec![
                String::from("my-cmd"),
                String::from("--arg-1"),
                String::from("--arg-1"),
            ],
            vec![String::from("arg-1")],
            String::from(""),
        );
    }

    #[test]
    #[should_panic]
    fn test_parser_overwriting_argvalue_no_value_panic() {
        parse_cli(
            vec![
                String::from("my-cmd"),
                String::from("--arg-1"),
                String::from("--arg-1"),
                String::from("arg-value-1"),
            ],
            vec![String::from("arg-1")],
            String::from(""),
        );
    }

    #[test]
    #[should_panic]
    fn test_parser_overwriting_value_with_no_value() {
        parse_cli(
            vec![
                String::from("my-cmd"),
                String::from("--arg-1"),
                String::from("arg-value-1"),
                String::from("--arg-1"),
                String::from("--arg-2"),
            ],
            vec![String::from("arg-1"), String::from("arg-2")],
            String::from(""),
        );
    }

    #[test]
    fn test_parser_correctly_parse_command_name() {
        let parsed_command = parse_cli(vec![String::from("my-cmd")], vec![], String::from("--"));
        assert_eq!(parsed_command.command_name, String::from("my-cmd"));
    }

    #[test]
    fn test_parser_correctly_parse_params() {
        let parsed_command = parse_cli(
            vec![
                String::from("my-cmd"),
                String::from("param-1"),
                String::from("param-2"),
            ],
            vec![],
            String::from("--"),
        );
        assert_eq!(parsed_command.params.len(), 2);
        assert_eq!(parsed_command.params[0], String::from("param-1"));
        assert_eq!(parsed_command.params[1], String::from("param-2"));
    }

    #[test]
    fn test_parser_correctly_parse_args_with_single_value() {
        let parsed_command = parse_cli(
            vec![
                String::from("my-cmd"),
                String::from("--arg-1"),
                String::from("value-1"),
            ],
            vec![String::from("arg-1")],
            String::from("--"),
        );
        assert_eq!(parsed_command.params.len(), 0);
        assert_eq!(parsed_command.args.len(), 1);
        assert_eq!(parsed_command.args[0].arg_name, "arg-1");
        assert_eq!(
            parsed_command.args[0].arg_value,
            ArgValue::Single(String::from("value-1"))
        );
    }

    #[test]
    fn test_parser_correctly_parse_two_args_with_single_value() {
        let parsed_command = parse_cli(
            vec![
                String::from("my-cmd"),
                String::from("--arg-1"),
                String::from("value-1"),
                String::from("--arg-2"),
                String::from("value-2"),
            ],
            vec![String::from("arg-1"), String::from("arg-2")],
            String::from("--"),
        );
        assert_eq!(parsed_command.params.len(), 0);
        assert_eq!(parsed_command.args.len(), 2);
        assert_eq!(parsed_command.args[0].arg_name, "arg-1");
        assert_eq!(
            parsed_command.args[0].arg_value,
            ArgValue::Single(String::from("value-1"))
        );
        assert_eq!(parsed_command.args[1].arg_name, "arg-2");
        assert_eq!(
            parsed_command.args[1].arg_value,
            ArgValue::Single(String::from("value-2"))
        );
    }

    #[test]
    fn test_parser_correctly_parse_args_with_multiple_values() {
        let parsed_command = parse_cli(
            vec![
                String::from("my-cmd"),
                String::from("--arg-1"),
                String::from("value-1"),
                String::from("--arg-1"),
                String::from("value-2"),
                String::from("--arg-1"),
                String::from("value-3"),
            ],
            vec![String::from("arg-1")],
            String::from("--"),
        );
        assert_eq!(parsed_command.params.len(), 0);
        assert_eq!(parsed_command.args.len(), 1);
        assert_eq!(parsed_command.args[0].arg_name, "arg-1");
        assert_eq!(
            parsed_command.args[0].arg_value,
            ArgValue::Multiple(vec![
                String::from("value-1"),
                String::from("value-2"),
                String::from("value-3")
            ])
        );
    }

    #[test]
    fn test_parser_correctly_parse_combinaison_of_singe_arg_and_multiple_arg() {
        let parsed_command = parse_cli(
            vec![
                String::from("my-cmd"),
                String::from("--arg-1"),
                String::from("value-1"),
                String::from("--arg-1"),
                String::from("value-2"),
                String::from("--arg-2"),
                String::from("value-3"),
            ],
            vec![String::from("arg-1"), String::from("arg-2")],
            String::from("--"),
        );
        assert_eq!(parsed_command.params.len(), 0);
        assert_eq!(parsed_command.args.len(), 2);
        assert_eq!(parsed_command.args[0].arg_name, "arg-1");
        assert_eq!(
            parsed_command.args[0].arg_value,
            ArgValue::Multiple(vec![String::from("value-1"), String::from("value-2")])
        );
        assert_eq!(parsed_command.args[1].arg_name, "arg-2");
        assert_eq!(
            parsed_command.args[1].arg_value,
            ArgValue::Single(String::from("value-3"))
        );
    }

    #[test]
    fn test_parser_correctly_parse_combinaison_of_singe_arg_multiple_arg_and_no_value_arg() {
        let parsed_command = parse_cli(
            vec![
                String::from("my-cmd"),
                String::from("--arg-2"),
                String::from("--arg-3"),
                String::from("value-1"),
                String::from("--arg-3"),
                String::from("value-2"),
                String::from("--arg-4"),
                String::from("value-3"),
            ],
            vec![
                String::from("arg-1"),
                String::from("arg-2"),
                String::from("arg-3"),
                String::from("arg-4"),
            ],
            String::from("--"),
        );
        assert_eq!(parsed_command.params.len(), 0);
        assert_eq!(parsed_command.args.len(), 4);
        assert_eq!(parsed_command.args[0].arg_name, "arg-1");
        assert_eq!(parsed_command.args[0].arg_value, ArgValue::NotProvided);
        assert_eq!(parsed_command.args[1].arg_name, "arg-2");
        assert_eq!(parsed_command.args[1].arg_value, ArgValue::NoValue);
        assert_eq!(parsed_command.args[2].arg_name, "arg-3");
        assert_eq!(
            parsed_command.args[2].arg_value,
            ArgValue::Multiple(vec![String::from("value-1"), String::from("value-2")])
        );
        assert_eq!(parsed_command.args[3].arg_name, "arg-4");
        assert_eq!(
            parsed_command.args[3].arg_value,
            ArgValue::Single(String::from("value-3"))
        );
    }

    #[test]
    fn test_parser_two_following_args_should_create_two_no_value_args() {
        let parsed_command = parse_cli(
            vec![
                String::from("my-cmd"),
                String::from("--arg-1"),
                String::from("--arg-2"),
            ],
            vec![String::from("arg-1"), String::from("arg-2")],
            String::from("--"),
        );

        assert_eq!(parsed_command.params.len(), 0);

        assert_eq!(parsed_command.args.len(), 2);
        assert_eq!(parsed_command.args[0].arg_name, String::from("arg-1"));
        assert_eq!(parsed_command.args[0].arg_value, ArgValue::NoValue);
        assert_eq!(parsed_command.args[1].arg_name, String::from("arg-2"));
        assert_eq!(parsed_command.args[1].arg_value, ArgValue::NoValue);
    }
}

mod cli_handler_tests {}

mod cli_handler_builder_tests {
    use crate::{
        cli_handler::{
            cli_handler_builder::{ArgValueTypes, CliHandlerBuilder},
            parser::ArgValue,
        },
        utils::logger::Logger,
    };

    #[test]
    #[should_panic = "[ERROR] [TEST_COMMAND] command specificatino needs to have a handler closure associated!"]
    fn test_missing_handler_in_command() {
        let logger = Logger::new("TEST_COMMAND", true);
        CliHandlerBuilder::new(logger)
            .command(
                String::from("my-command"),
                String::from("my description"),
                Logger::new("my-command", true),
            )
            .build();

        // Should have fail before.
        assert!(false);
    }

    #[test]
    #[should_panic = "[ERROR] [TEST_COMMAND] handler should only be set once per command."]
    fn test_two_handler_in_command() {
        let logger = Logger::new("TEST_COMMAND", true);
        CliHandlerBuilder::new(logger)
            .command(
                String::from("my-command"),
                String::from("my description"),
                Logger::new("my-command", true),
            )
            .handler(|_| ())
            .handler(|_| ())
            .build();

        // Should have fail before.
        assert!(false);
    }

    #[test]
    #[should_panic = "[ERROR] [TEST_COMMAND] no command has been specified."]
    fn test_no_command_specified_when_building() {
        let logger = Logger::new("TEST_COMMAND", true);
        CliHandlerBuilder::new(logger).build();

        // Should have fail before.
        assert!(false);
    }

    #[test]
    fn test_can_create_command() {
        let logger = Logger::new("TEST_COMMAND", true);
        let cli_handler = CliHandlerBuilder::new(logger)
            .command(
                String::from("my-command"),
                String::from("my description"),
                Logger::new("my-command", true),
            )
            .handler(|_| ())
            .build();

        assert_eq!(cli_handler.command_handlers.len(), 1);
        assert_eq!(
            cli_handler.command_handlers[0].command_name,
            String::from("my-command")
        );
        assert_eq!(
            cli_handler.command_handlers[0].command_description,
            String::from("my description")
        );
        assert_eq!(cli_handler.command_handlers[0].args.len(), 0);
        assert_eq!(cli_handler.command_handlers[0].params.len(), 0);
    }

    #[test]
    fn test_can_add_one_arg_to_command() {
        let logger = Logger::new("TEST_COMMAND", true);
        let cli_handler = CliHandlerBuilder::new(logger)
            .command(
                String::from("my-command"),
                String::from("my description"),
                Logger::new("my-command", true),
            )
            .args(
                String::from("arg-1"),
                String::from("description of arg-1"),
                vec![ArgValueTypes::NoValue],
            )
            .handler(|_| ())
            .build();

        assert_eq!(cli_handler.command_handlers.len(), 1);

        let args = cli_handler.command_handlers[0].args.clone();
        assert_eq!(args.len(), 1);
        assert_eq!(args[0].name, String::from("arg-1"));
        assert_eq!(args[0].description, String::from("description of arg-1"));
        assert_eq!(args[0].expected_value_type.len(), 1);
        assert_eq!(args[0].expected_value_type[0], ArgValueTypes::NoValue);
    }

    #[test]
    fn test_specified_arg_can_hold_multiple_expected_values() {
        let logger = Logger::new("TEST_COMMAND", true);
        let cli_handler = CliHandlerBuilder::new(logger)
            .command(
                String::from("my-command"),
                String::from("my description"),
                Logger::new("my-command", true),
            )
            .args(
                String::from("arg-1"),
                String::from("description of arg-1"),
                vec![ArgValueTypes::NoValue, ArgValueTypes::Single],
            )
            .handler(|_| ())
            .build();

        assert_eq!(cli_handler.command_handlers.len(), 1);

        let args = cli_handler.command_handlers[0].args.clone();
        assert_eq!(args.len(), 1);
        assert_eq!(args[0].name, String::from("arg-1"));
        assert_eq!(args[0].description, String::from("description of arg-1"));
        assert_eq!(args[0].expected_value_type.len(), 2);
        assert_eq!(args[0].expected_value_type[0], ArgValueTypes::NoValue);
        assert_eq!(args[0].expected_value_type[1], ArgValueTypes::Single);
    }

    #[test]
    fn test_can_add_multiple_arg_to_command() {
        let logger = Logger::new("TEST_COMMAND", true);
        let cli_handler = CliHandlerBuilder::new(logger)
            .command(
                String::from("my-command"),
                String::from("my description"),
                Logger::new("my-command", true),
            )
            .args(
                String::from("arg-1"),
                String::from("description of arg-1"),
                vec![ArgValueTypes::NoValue],
            )
            .args(
                String::from("arg-2"),
                String::from("description of arg-2"),
                vec![ArgValueTypes::Single, ArgValueTypes::Multiple],
            )
            .handler(|_| ())
            .build();

        assert_eq!(cli_handler.command_handlers.len(), 1);

        let args = cli_handler.command_handlers[0].args.clone();
        assert_eq!(args.len(), 2);

        // test for arg-1
        assert_eq!(args[0].name, String::from("arg-1"));
        assert_eq!(args[0].description, String::from("description of arg-1"));
        assert_eq!(args[0].expected_value_type.len(), 1);
        assert_eq!(args[0].expected_value_type[0], ArgValueTypes::NoValue);

        // test for arg-2
        assert_eq!(args[1].name, String::from("arg-2"));
        assert_eq!(args[1].description, String::from("description of arg-2"));
        assert_eq!(args[1].expected_value_type.len(), 2);
        assert_eq!(args[1].expected_value_type[0], ArgValueTypes::Single);
        assert_eq!(args[1].expected_value_type[1], ArgValueTypes::Multiple);
    }

    #[test]
    #[should_panic = "[ERROR] [TEST_COMMAND] argument 'arg-1' have already been declared for command 'my-command'."]
    fn test_error_when_arg_specification_is_duplicated_for_same_command() {
        let logger = Logger::new("TEST_COMMAND", true);
        CliHandlerBuilder::new(logger)
            .command(
                String::from("my-command"),
                String::from("my description"),
                Logger::new("my-command", true),
            )
            .args(
                String::from("arg-1"),
                String::from("This is a description of arg-1"),
                vec![ArgValueTypes::NoValue],
            )
            .args(
                String::from("arg-1"),
                String::from("This is a description of arg-1"),
                vec![ArgValueTypes::NoValue],
            )
            .handler(|_| ())
            .build();

        assert!(false);
    }

    #[test]
    #[should_panic = "[ERROR] [TEST_COMMAND] args method cannot receive empty 'expected_value_type' vector."]
    fn test_error_when_arg_specification_receive_empty_expected_value_list() {
        let logger = Logger::new("TEST_COMMAND", true);
        CliHandlerBuilder::new(logger)
            .command(
                String::from("my-command"),
                String::from("my description"),
                Logger::new("my-command", true),
            )
            .args(
                String::from("arg-1"),
                String::from("This is a description of arg-1"),
                vec![],
            )
            .handler(|_| ())
            .build();

        assert!(false);
    }

    #[test]
    #[should_panic = "[ERROR] [TEST_COMMAND] cannot specify argument 'arg-1' outside of command context."]
    fn test_error_when_arg_is_declare_outside_of_command_context() {
        let logger = Logger::new("TEST_COMMAND", true);
        CliHandlerBuilder::new(logger)
            .args(
                String::from("arg-1"),
                String::from("This is a description of arg-1"),
                vec![ArgValueTypes::NoValue],
            )
            .build();

        assert!(false);
    }

    #[test]
    fn test_can_add_one_param_to_command() {
        let logger = Logger::new("TEST_COMMAND", true);
        let cli_handler = CliHandlerBuilder::new(logger)
            .command(
                String::from("my-command"),
                String::from("my description"),
                Logger::new("my-command", true),
            )
            .params(
                String::from("param-1"),
                String::from("description of param-1"),
            )
            .handler(|_| ())
            .build();

        assert_eq!(cli_handler.command_handlers.len(), 1);

        let params = cli_handler.command_handlers[0].params.clone();
        assert_eq!(params.len(), 1);
        assert_eq!(params[0].name, String::from("param-1"));
        assert_eq!(params[0].description, String::from("description of param-1"));
    }

    #[test]
    fn test_can_add_multiple_param_to_command() {
        let logger = Logger::new("TEST_COMMAND", true);
        let cli_handler = CliHandlerBuilder::new(logger)
            .command(
                String::from("my-command"),
                String::from("my description"),
                Logger::new("my-command", true),
            )
            .params(
                String::from("param-1"),
                String::from("description of param-1"),
            )
            .params(
                String::from("param-2"),
                String::from("description of param-2"),
            )
            .handler(|_| ())
            .build();

        assert_eq!(cli_handler.command_handlers.len(), 1);

        let params = cli_handler.command_handlers[0].params.clone();
        assert_eq!(params.len(), 2);
        assert_eq!(params[0].name, String::from("param-1"));
        assert_eq!(params[0].description, String::from("description of param-1"));
        assert_eq!(params[1].name, String::from("param-2"));
        assert_eq!(params[1].description, String::from("description of param-2"));
    }

    #[test]
    #[should_panic = "[ERROR] [TEST_COMMAND] parameter 'param-1' have already been declared for command 'my-command'."]
    fn test_error_when_param_specification_is_duplicated_for_same_command() {
        let logger = Logger::new("TEST_COMMAND", true);
        CliHandlerBuilder::new(logger)
            .command(
                String::from("my-command"),
                String::from("my description"),
                Logger::new("my-command", true),
            )
            .params(
                String::from("param-1"),
                String::from("This is a description of param-1"),
            )
            .params(
                String::from("param-1"),
                String::from("This is a description of param-1"),
            )
            .handler(|_| ())
            .build();

        assert!(false);
    }

    #[test]
    #[should_panic = "[ERROR] [TEST_COMMAND] cannot specify parameter 'param-1' outside of command context."]
    fn test_error_when_param_is_declare_outside_of_command_context() {
        let logger = Logger::new("TEST_COMMAND", true);
        CliHandlerBuilder::new(logger)
            .params(
                String::from("param-1"),
                String::from("This is a description of param-1"),
            )
            .build();

        assert!(false);
    }
}
