#[cfg(test)]
mod parser_tests {
    use crate::{
        cli::cli_handler::{
            cli_handler::CliHandlerCommand,
            cli_handler_builder::ArgValueTypes,
            compound_structs::ArgBuilder,
            parser::{parse_cli, ArgDatum, ArgValue},
        },
        utils::logger::Logger,
    };

    fn get_handler_for_expected_args(expected_args: Vec<String>) -> CliHandlerCommand {
        CliHandlerCommand {
            logger: Logger::new("name", true),
            command_name: "command name".to_string(),
            command_description: String::from("Test command handler decription"),
            args: expected_args
                .iter()
                .map(|arg_name| ArgBuilder {
                    name: arg_name.clone(),
                    description: String::from(format!("description for {arg_name}")),
                    expected_value_type: vec![ArgValueTypes::Single, ArgValueTypes::Multiple],
                    parent_name: None,
                })
                .collect(),
            params: vec![],
            handler: |_, _| (),
        }
    }

    #[test]
    #[should_panic(expected = "[Cli Parser] Expect to receive non-empty command input.")]
    fn test_parser_no_command_panic() {
        let command_handler = get_handler_for_expected_args(vec![]);
        parse_cli(vec![], &command_handler, String::from(""));
    }

    #[test]
    fn test_parser_correctly_parse_command_name() {
        let command_handler = get_handler_for_expected_args(vec![]);
        let parsed_command = parse_cli(
            vec![String::from("my-cmd")],
            &command_handler,
            String::from("--"),
        )
        .unwrap();
        assert_eq!(parsed_command.command_name, String::from("my-cmd"));
    }

    #[test]
    fn test_parser_correctly_parse_params() {
        let command_handler = get_handler_for_expected_args(vec![]);
        let parsed_command = parse_cli(
            vec![
                String::from("my-cmd"),
                String::from("param-1"),
                String::from("param-2"),
            ],
            &command_handler,
            String::from("--"),
        )
        .unwrap();
        assert_eq!(parsed_command.params.len(), 2);
        assert_eq!(parsed_command.params[0], String::from("param-1"));
        assert_eq!(parsed_command.params[1], String::from("param-2"));
    }

    #[test]
    fn test_parser_correctly_parse_args_with_single_value() {
        let command_handler = get_handler_for_expected_args(vec![String::from("arg-1")]);
        let parsed_command = parse_cli(
            vec![
                String::from("my-cmd"),
                String::from("--arg-1"),
                String::from("value-1"),
            ],
            &command_handler,
            String::from("--"),
        )
        .unwrap();
        assert_eq!(parsed_command.params.len(), 0);
        assert_eq!(parsed_command.args.len(), 1);
        assert_eq!(parsed_command.args[0].arg_name, "arg-1");
        assert_eq!(
            parsed_command.args[0].arg_value,
            ArgValue::Single(ArgDatum {
                value: Some(String::from("value-1")),
                child_args: vec![]
            })
        );
    }

    #[test]
    fn test_parser_correctly_parse_two_args_with_single_value() {
        let command_handler =
            get_handler_for_expected_args(vec![String::from("arg-1"), String::from("arg-2")]);
        let parsed_command = parse_cli(
            vec![
                String::from("my-cmd"),
                String::from("--arg-1"),
                String::from("value-1"),
                String::from("--arg-2"),
                String::from("value-2"),
            ],
            &command_handler,
            String::from("--"),
        )
        .unwrap();
        assert_eq!(parsed_command.params.len(), 0);
        assert_eq!(parsed_command.args.len(), 2);
        assert_eq!(parsed_command.args[0].arg_name, "arg-1");
        assert_eq!(
            parsed_command.args[0].arg_value,
            ArgValue::Single(ArgDatum {
                value: Some(String::from("value-1")),
                child_args: vec![]
            })
        );
        assert_eq!(parsed_command.args[1].arg_name, "arg-2");
        assert_eq!(
            parsed_command.args[1].arg_value,
            ArgValue::Single(ArgDatum {
                value: Some(String::from("value-2")),
                child_args: vec![]
            })
        );
    }

    #[test]
    fn test_parser_correctly_parse_args_with_multiple_values() {
        let command_handler = get_handler_for_expected_args(vec![String::from("arg-1")]);
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
            &command_handler,
            String::from("--"),
        )
        .unwrap();
        assert_eq!(parsed_command.params.len(), 0);
        assert_eq!(parsed_command.args.len(), 1);
        assert_eq!(parsed_command.args[0].arg_name, "arg-1");
        assert_eq!(
            parsed_command.args[0].arg_value,
            ArgValue::Multiple(
                vec![
                    String::from("value-1"),
                    String::from("value-2"),
                    String::from("value-3")
                ]
                .iter()
                .map(|value| {
                    ArgDatum {
                        value: Some(value.to_string()),
                        child_args: vec![],
                    }
                })
                .collect()
            )
        );
    }

    #[test]
    fn test_parser_correctly_parse_combinaison_of_singe_arg_and_multiple_arg() {
        let command_handler =
            get_handler_for_expected_args(vec![String::from("arg-1"), String::from("arg-2")]);
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
            &command_handler,
            String::from("--"),
        )
        .unwrap();
        assert_eq!(parsed_command.params.len(), 0);
        assert_eq!(parsed_command.args.len(), 2);
        assert_eq!(parsed_command.args[0].arg_name, "arg-1");
        assert_eq!(
            parsed_command.args[0].arg_value,
            ArgValue::Multiple(
                vec![String::from("value-1"), String::from("value-2")]
                    .iter()
                    .map(|value| {
                        ArgDatum {
                            value: Some(value.to_string()),
                            child_args: vec![],
                        }
                    })
                    .collect()
            )
        );
        assert_eq!(parsed_command.args[1].arg_name, "arg-2");
        assert_eq!(
            parsed_command.args[1].arg_value,
            ArgValue::Single(ArgDatum {
                value: Some(String::from("value-3")),
                child_args: vec![]
            })
        );
    }

    #[test]
    fn test_parser_correctly_parse_combinaison_of_singe_arg_multiple_arg_and_no_value_arg() {
        let command_handler = get_handler_for_expected_args(vec![
            String::from("arg-1"),
            String::from("arg-2"),
            String::from("arg-3"),
            String::from("arg-4"),
        ]);
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
            &command_handler,
            String::from("--"),
        )
        .unwrap();
        println!("{:?}", parsed_command.args);
        assert_eq!(parsed_command.params.len(), 0);
        assert_eq!(parsed_command.args.len(), 4);
        assert_eq!(parsed_command.args[0].arg_name, "arg-2");
        assert_eq!(
            parsed_command.args[0].arg_value,
            ArgValue::Single(ArgDatum {
                value: None,
                child_args: vec![]
            })
        );
        assert_eq!(parsed_command.args[1].arg_name, "arg-3");
        assert_eq!(
            parsed_command.args[1].arg_value,
            ArgValue::Multiple(
                vec![String::from("value-1"), String::from("value-2")]
                    .iter()
                    .map(|value| {
                        ArgDatum {
                            value: Some(value.to_string()),
                            child_args: vec![],
                        }
                    })
                    .collect()
            )
        );
        assert_eq!(parsed_command.args[2].arg_name, "arg-4");
        assert_eq!(
            parsed_command.args[2].arg_value,
            ArgValue::Single(ArgDatum {
                value: Some(String::from("value-3")),
                child_args: vec![]
            })
        );
        assert_eq!(parsed_command.args[3].arg_name, "arg-1");
        assert_eq!(parsed_command.args[3].arg_value, ArgValue::NotProvided);
    }

    #[test]
    fn test_parser_two_following_args_should_create_two_no_value_args() {
        let command_handler =
            get_handler_for_expected_args(vec![String::from("arg-1"), String::from("arg-2")]);
        let parsed_command = parse_cli(
            vec![
                String::from("my-cmd"),
                String::from("--arg-1"),
                String::from("--arg-2"),
            ],
            &command_handler,
            String::from("--"),
        )
        .unwrap();

        assert_eq!(parsed_command.params.len(), 0);

        assert_eq!(parsed_command.args.len(), 2);
        assert_eq!(parsed_command.args[0].arg_name, String::from("arg-1"));
        assert_eq!(
            parsed_command.args[0].arg_value,
            ArgValue::Single(ArgDatum {
                value: None,
                child_args: vec![]
            })
        );
        assert_eq!(parsed_command.args[1].arg_name, String::from("arg-2"));
        assert_eq!(
            parsed_command.args[1].arg_value,
            ArgValue::Single(ArgDatum {
                value: None,
                child_args: vec![]
            })
        );
    }
}

#[cfg(test)]
mod cli_handler_builder_tests {
    use crate::{
        cli::cli_handler::cli_handler_builder::{ArgValueTypes, CliHandlerBuilder},
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
            .handler(|_, _| ())
            .handler(|_, _| ())
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
            .handler(|_, _| ())
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
            .argument(
                String::from("arg-1"),
                String::from("description of arg-1"),
                vec![ArgValueTypes::NoValue],
            )
            .handler(|_, _| ())
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
            .argument(
                String::from("arg-1"),
                String::from("description of arg-1"),
                vec![ArgValueTypes::NoValue, ArgValueTypes::Single],
            )
            .handler(|_, _| ())
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
            .argument(
                String::from("arg-1"),
                String::from("description of arg-1"),
                vec![ArgValueTypes::NoValue],
            )
            .argument(
                String::from("arg-2"),
                String::from("description of arg-2"),
                vec![ArgValueTypes::Single, ArgValueTypes::Multiple],
            )
            .handler(|_, _| ())
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
            .argument(
                String::from("arg-1"),
                String::from("This is a description of arg-1"),
                vec![ArgValueTypes::NoValue],
            )
            .argument(
                String::from("arg-1"),
                String::from("This is a description of arg-1"),
                vec![ArgValueTypes::NoValue],
            )
            .handler(|_, _| ())
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
            .argument(
                String::from("arg-1"),
                String::from("This is a description of arg-1"),
                vec![],
            )
            .handler(|_, _| ())
            .build();

        assert!(false);
    }

    #[test]
    #[should_panic = "[ERROR] [TEST_COMMAND] cannot specify argument 'arg-1' outside of command context."]
    fn test_error_when_arg_is_declare_outside_of_command_context() {
        let logger = Logger::new("TEST_COMMAND", true);
        CliHandlerBuilder::new(logger)
            .argument(
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
            .parameter(
                String::from("param-1"),
                String::from("description of param-1"),
            )
            .handler(|_, _| ())
            .build();

        assert_eq!(cli_handler.command_handlers.len(), 1);

        let params = cli_handler.command_handlers[0].params.clone();
        assert_eq!(params.len(), 1);
        assert_eq!(params[0].name, String::from("param-1"));
        assert_eq!(
            params[0].description,
            String::from("description of param-1")
        );
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
            .parameter(
                String::from("param-1"),
                String::from("description of param-1"),
            )
            .parameter(
                String::from("param-2"),
                String::from("description of param-2"),
            )
            .handler(|_, _| ())
            .build();

        assert_eq!(cli_handler.command_handlers.len(), 1);

        let params = cli_handler.command_handlers[0].params.clone();
        assert_eq!(params.len(), 2);
        assert_eq!(params[0].name, String::from("param-1"));
        assert_eq!(
            params[0].description,
            String::from("description of param-1")
        );
        assert_eq!(params[1].name, String::from("param-2"));
        assert_eq!(
            params[1].description,
            String::from("description of param-2")
        );
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
            .parameter(
                String::from("param-1"),
                String::from("This is a description of param-1"),
            )
            .parameter(
                String::from("param-1"),
                String::from("This is a description of param-1"),
            )
            .handler(|_, _| ())
            .build();

        assert!(false);
    }

    #[test]
    #[should_panic = "[ERROR] [TEST_COMMAND] cannot specify parameter 'param-1' outside of command context."]
    fn test_error_when_param_is_declare_outside_of_command_context() {
        let logger = Logger::new("TEST_COMMAND", true);
        CliHandlerBuilder::new(logger)
            .parameter(
                String::from("param-1"),
                String::from("This is a description of param-1"),
            )
            .build();

        assert!(false);
    }

    #[test]
    fn test_specification_multiple_commands() {
        let logger = Logger::new("TEST_COMMAND", true);
        let cli_handler = CliHandlerBuilder::new(logger)
            .command(
                String::from("my-command"),
                String::from("my description"),
                Logger::new("my-command", true),
            )
            .handler(|_, _| ())
            .command(
                String::from("my-command-2"),
                String::from("my description for my-command-2"),
                Logger::new("my-command-2", true),
            )
            .handler(|_, _| ())
            .build();

        assert_eq!(cli_handler.command_handlers.len(), 2);
    }
}

#[cfg(test)]
mod cli_handler_tests {
    use crate::{
        cli::cli_handler::{
            cli_handler_builder::{ArgValueTypes, CliHandlerBuilder},
            parser::{ArgDatum, ArgValue, ParsedArgs},
        },
        utils::logger::Logger,
    };

    #[test]
    #[should_panic = "[ERROR] [TEST_COMMAND] please provide a valid command."]
    fn test_no_command_entered() {
        let cli_handler = CliHandlerBuilder::new(Logger::new("TEST_COMMAND", true))
            .command(
                String::from("command-1"),
                String::from("description"),
                Logger::new("command-1 log", false),
            )
            .handler(|_, _| {})
            .build();
        cli_handler.handle(String::from(""));
    }

    #[test]
    #[should_panic = "[ERROR] [TEST_COMMAND] 'unknown-command' isn't a valid command. Please type 'help' to get the list of valid commands."]
    fn test_command_not_found() {
        let cli_handler = CliHandlerBuilder::new(Logger::new("TEST_COMMAND", true))
            .command(
                String::from("command-1"),
                String::from("description"),
                Logger::new("command-1 log", false),
            )
            .handler(|_, _| {})
            .build();
        cli_handler.handle(String::from("unknown-command"));
    }

    #[test]
    #[should_panic = "[ERROR] [my-command] unknown argument: got 'arg-unknown' when expecting arg-1, arg-2. See 'help' to get more informations."]
    fn test_received_unknown_argument() {
        let cli_handler = CliHandlerBuilder::new(Logger::new("CLI LOG", true))
            .command(
                String::from("my-command"),
                String::from("description"),
                Logger::new("my-command", true),
            )
            .argument(
                String::from("arg-1"),
                String::from("desc for arg-1"),
                vec![ArgValueTypes::NoValue],
            )
            .argument(
                String::from("arg-2"),
                String::from("desc for arg-2"),
                vec![ArgValueTypes::Single, ArgValueTypes::Multiple],
            )
            .handler(|_, _| {})
            .build();
        cli_handler.handle(String::from("my-command --arg-unknown"));
    }

    #[test]
    #[should_panic = "[ERROR] [my-command] unexpected argument value for arg-1: expected Single, Multiple but received NoValue."]
    fn test_received_unexpected_argument_value_got_no_value() {
        let cli_handler = CliHandlerBuilder::new(Logger::new("CLI LOG", true))
            .command(
                String::from("my-command"),
                String::from("description"),
                Logger::new("my-command", true),
            )
            .argument(
                String::from("arg-1"),
                String::from("desc for arg-1"),
                vec![ArgValueTypes::Single, ArgValueTypes::Multiple],
            )
            .handler(|_, _| {})
            .build();
        cli_handler.handle(String::from("my-command --arg-1"));
    }

    #[test]
    #[should_panic = "[ERROR] [my-command] unexpected argument value for arg-1: expected NoValue, Multiple but received Single."]
    fn test_received_unexpected_argument_value_got_single_value() {
        let cli_handler = CliHandlerBuilder::new(Logger::new("CLI LOG", true))
            .command(
                String::from("my-command"),
                String::from("description"),
                Logger::new("my-command", true),
            )
            .argument(
                String::from("arg-1"),
                String::from("desc for arg-1"),
                vec![ArgValueTypes::NoValue, ArgValueTypes::Multiple],
            )
            .handler(|_, _| {})
            .build();
        cli_handler.handle(String::from("my-command --arg-1 test"));
    }

    #[test]
    #[should_panic = "[ERROR] [my-command] unexpected argument value for arg-1: expected NoValue, Single but received Multiple."]
    fn test_received_unexpected_argument_value_got_multiple_value() {
        let cli_handler = CliHandlerBuilder::new(Logger::new("CLI LOG", true))
            .command(
                String::from("my-command"),
                String::from("description"),
                Logger::new("my-command", true),
            )
            .argument(
                String::from("arg-1"),
                String::from("desc for arg-1"),
                vec![ArgValueTypes::NoValue, ArgValueTypes::Single],
            )
            .handler(|_, _| {})
            .build();
        cli_handler.handle(String::from("my-command --arg-1 test --arg-1 test2"));
    }

    #[test]
    #[should_panic = "[ERROR] [my-command] too much parameters: expected 2 parameters but received 3."]
    fn test_error_too_much_parameters() {
        let cli_handler = CliHandlerBuilder::new(Logger::new("CLI LOG", true))
            .command(
                String::from("my-command"),
                String::from("description"),
                Logger::new("my-command", true),
            )
            .parameter(String::from("param-1-name"), String::from("param-1-desc"))
            .parameter(String::from("param-2-name"), String::from("param-2-desc"))
            .handler(|_, _| {})
            .build();
        cli_handler.handle(String::from("my-command param-1 param-2 param-3"));
    }

    #[test]
    #[should_panic = "[ERROR] [my-command] too much parameters: expected 0 parameters but received 1."]
    fn test_error_too_much_parameters_no_parameters_specified() {
        let cli_handler = CliHandlerBuilder::new(Logger::new("CLI LOG", true))
            .command(
                String::from("my-command"),
                String::from("description"),
                Logger::new("my-command", true),
            )
            .handler(|_, _| {})
            .build();
        cli_handler.handle(String::from("my-command param-1"));
    }

    #[test]
    #[should_panic = "[ERROR] [my-command] not enough parameters: expected 2 parameters but received 1."]
    fn test_error_not_enough_parameters_some_parameters_specified() {
        let cli_handler = CliHandlerBuilder::new(Logger::new("CLI LOG", true))
            .command(
                String::from("my-command"),
                String::from("description"),
                Logger::new("my-command", true),
            )
            .parameter(String::from("param-1-name"), String::from("param-1-desc"))
            .parameter(String::from("param-2-name"), String::from("param-2-desc"))
            .handler(|_, _| {})
            .build();
        cli_handler.handle(String::from("my-command param-1"));
    }

    #[test]
    // We use should_panic to insure that callback is executed
    #[should_panic = "handler executed"]
    fn test_correctly_call_handler_when_input_valid() {
        let cli_handler = CliHandlerBuilder::new(Logger::new("CLI LOG", true))
            .command(
                String::from("my-command"),
                String::from("description"),
                Logger::new("my-command", true),
            )
            .argument(
                String::from("arg-1"),
                String::from("desc for arg-1"),
                vec![ArgValueTypes::NoValue],
            )
            .argument(
                String::from("arg-2"),
                String::from("desc for arg-1"),
                vec![ArgValueTypes::Single],
            )
            .parameter(String::from("params-1"), String::from("desc for params-1"))
            // It's hard to test the fact that the handler will be called.
            // TODO: That's a futur me assignment.
            .handler(|parsed_command, _| {
                // Here we want to check that the correct parsed_command is passed to the handler.
                // A more intense is done in the `parser_tests` module (see above)
                assert_eq!(parsed_command.command_name, String::from("my-command"));
                assert_eq!(parsed_command.args.len(), 2);
                assert_eq!(parsed_command.args[0].arg_name, String::from("arg-1"));
                assert_eq!(
                    parsed_command.args[0].arg_value,
                    ArgValue::Single(ArgDatum {
                        value: None,
                        child_args: vec![]
                    })
                );
                assert_eq!(parsed_command.args[1].arg_name, String::from("arg-2"));
                assert_eq!(
                    parsed_command.args[1].arg_value,
                    ArgValue::Single(ArgDatum {
                        value: Some(String::from("arg-2-value")),
                        child_args: vec![]
                    })
                );
                assert_eq!(parsed_command.params.len(), 1);
                assert_eq!(parsed_command.params[0], String::from("param-1-value"));

                panic!("handler executed");
            })
            .build();

        cli_handler.handle(String::from(
            "my-command --arg-1 --arg-2 arg-2-value param-1-value",
        ));
    }

    #[test]
    fn test_correctly_parsed_nested_args() {
        let cli_handler = CliHandlerBuilder::new(Logger::new("TEST_COMMAND", true))
            .command(
                String::from("my-command"),
                String::from("my-command's description"),
                Logger::new("my-command", true),
            )
            .argument(
                String::from("arg-1"),
                String::from("desc for arg-1"),
                vec![ArgValueTypes::Multiple],
            )
            .linked_arg(
                String::from("arg-2"),
                String::from("desc for arg-2"),
                vec![ArgValueTypes::Single, ArgValueTypes::Multiple],
            )
            .parameter(String::from("params-1"), String::from("desc for params-1"))
            .handler(|parsed_command, _| {
                assert_eq!(parsed_command.command_name, "my-command");

                // assert params
                assert_eq!(parsed_command.params.len(), 1);
                assert_eq!(parsed_command.params[0], "my-param");

                // assert arguments non-nested number
                assert_eq!(parsed_command.args.len(), 2);

                // assert first argument
                let first_arg = &parsed_command.args[0];
                assert_eq!(first_arg.arg_name, "arg-1");
                assert_eq!(
                    first_arg.arg_value,
                    ArgValue::Multiple(vec![
                        ArgDatum {
                            value: None,
                            child_args: vec![ParsedArgs {
                                arg_name: String::from("arg-2"),
                                arg_value: ArgValue::Single(ArgDatum {
                                    value: Some(String::from("value1")),
                                    child_args: vec![]
                                })
                            }]
                        },
                        ArgDatum {
                            value: Some(String::from("my-arg-1-value")),
                            child_args: vec![ParsedArgs {
                                arg_name: String::from("arg-2"),
                                arg_value: ArgValue::Multiple(vec![
                                    ArgDatum {
                                        value: Some(String::from("value2")),
                                        child_args: vec![]
                                    },
                                    ArgDatum {
                                        value: Some(String::from("value3")),
                                        child_args: vec![]
                                    }
                                ])
                            }]
                        }
                    ])
                );
            })
            .build();

        let input = String::from(
            "my-command my-param --arg-1 --arg-2 value1 --arg-1 my-arg-1-value --arg-2 value2 --arg-2 value3",
        );
        cli_handler.handle(input);
    }
}
