use bevy::prelude::*;

pub struct CardPlugin;

impl Plugin for CardPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(create_card_system.system());
    }
}

fn create_card_system(mut commands: Commands) {
    // commands
    //     .spawn()
    //     .insert(create_card(Vec2::new(200.0, 200.0)))
    //     .insert(Code {
    //         node: language::semantic::ir::CodeNode::Operate {
    //             operator_id: OperatorId(create_consistent_id("desk-protocol", "plus")),
    //             operands: vec![
    //                 Code {
    //                     node: language::semantic::ir::CodeNode::Literal {
    //                         literal_value: LiteralValue::I32(1),
    //                     },
    //                     return_type: Type::Basic {
    //                         basic_type_id: BasicTypeId(create_consistent_id("cargo-desk", "i32")),
    //                         parameters: Vec::new(),
    //                     },
    //                 },
    //                 Code {
    //                     node: language::semantic::ir::CodeNode::Literal {
    //                         literal_value: LiteralValue::I32(1),
    //                     },
    //                     return_type: Type::Basic {
    //                         basic_type_id: BasicTypeId(create_consistent_id("cargo-desk", "i32")),
    //                         parameters: Vec::new(),
    //                     },
    //                 },
    //             ],
    //         },
    //         return_type: Type::Basic {
    //             basic_type_id: BasicTypeId(create_consistent_id("cargo-desk", "i32")),
    //             parameters: Vec::new(),
    //         },
    //     });

    // commands
    //     .spawn()
    //     .insert(create_card(Vec2::new(400.0, 200.0)))
    //     .insert(Code {
    //         node: language::semantic::ir::CodeNode::Literal {
    //             literal_value: LiteralValue::String("Hello World".to_string()),
    //         },
    //         return_type: Type::Basic {
    //             basic_type_id: BasicTypeId(create_consistent_id("cargo-desk", "string")),
    //             parameters: Vec::new(),
    //         },
    //     });
}
