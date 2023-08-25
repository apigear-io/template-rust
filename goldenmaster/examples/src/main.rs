#![allow(unused_imports)]
extern crate testbed2;
extern crate tb_enum;
extern crate tb_same1;
extern crate tb_same2;
extern crate tb_simple;
extern crate testbed1;

// importing traits and allowing unused_imports for easy use below
use testbed2::api::many_param_interface::ManyParamInterfaceTrait as testbed2ManyParamInterfaceTrait;
use testbed2::implementation::many_param_interface::ManyParamInterface as testbed2ManyParamInterface;
use testbed2::api::nested_struct1_interface::NestedStruct1InterfaceTrait as testbed2NestedStruct1InterfaceTrait;
use testbed2::implementation::nested_struct1_interface::NestedStruct1Interface as testbed2NestedStruct1Interface;
use testbed2::api::nested_struct2_interface::NestedStruct2InterfaceTrait as testbed2NestedStruct2InterfaceTrait;
use testbed2::implementation::nested_struct2_interface::NestedStruct2Interface as testbed2NestedStruct2Interface;
use testbed2::api::nested_struct3_interface::NestedStruct3InterfaceTrait as testbed2NestedStruct3InterfaceTrait;
use testbed2::implementation::nested_struct3_interface::NestedStruct3Interface as testbed2NestedStruct3Interface;
use tb_enum::api::enum_interface::EnumInterfaceTrait as tb_enumEnumInterfaceTrait;
use tb_enum::implementation::enum_interface::EnumInterface as tb_enumEnumInterface;
use tb_same1::api::same_struct1_interface::SameStruct1InterfaceTrait as tb_same1SameStruct1InterfaceTrait;
use tb_same1::implementation::same_struct1_interface::SameStruct1Interface as tb_same1SameStruct1Interface;
use tb_same1::api::same_struct2_interface::SameStruct2InterfaceTrait as tb_same1SameStruct2InterfaceTrait;
use tb_same1::implementation::same_struct2_interface::SameStruct2Interface as tb_same1SameStruct2Interface;
use tb_same1::api::same_enum1_interface::SameEnum1InterfaceTrait as tb_same1SameEnum1InterfaceTrait;
use tb_same1::implementation::same_enum1_interface::SameEnum1Interface as tb_same1SameEnum1Interface;
use tb_same1::api::same_enum2_interface::SameEnum2InterfaceTrait as tb_same1SameEnum2InterfaceTrait;
use tb_same1::implementation::same_enum2_interface::SameEnum2Interface as tb_same1SameEnum2Interface;
use tb_same2::api::same_struct1_interface::SameStruct1InterfaceTrait as tb_same2SameStruct1InterfaceTrait;
use tb_same2::implementation::same_struct1_interface::SameStruct1Interface as tb_same2SameStruct1Interface;
use tb_same2::api::same_struct2_interface::SameStruct2InterfaceTrait as tb_same2SameStruct2InterfaceTrait;
use tb_same2::implementation::same_struct2_interface::SameStruct2Interface as tb_same2SameStruct2Interface;
use tb_same2::api::same_enum1_interface::SameEnum1InterfaceTrait as tb_same2SameEnum1InterfaceTrait;
use tb_same2::implementation::same_enum1_interface::SameEnum1Interface as tb_same2SameEnum1Interface;
use tb_same2::api::same_enum2_interface::SameEnum2InterfaceTrait as tb_same2SameEnum2InterfaceTrait;
use tb_same2::implementation::same_enum2_interface::SameEnum2Interface as tb_same2SameEnum2Interface;
use tb_simple::api::void_interface::VoidInterfaceTrait as tb_simpleVoidInterfaceTrait;
use tb_simple::implementation::void_interface::VoidInterface as tb_simpleVoidInterface;
use tb_simple::api::simple_interface::SimpleInterfaceTrait as tb_simpleSimpleInterfaceTrait;
use tb_simple::implementation::simple_interface::SimpleInterface as tb_simpleSimpleInterface;
use tb_simple::api::simple_array_interface::SimpleArrayInterfaceTrait as tb_simpleSimpleArrayInterfaceTrait;
use tb_simple::implementation::simple_array_interface::SimpleArrayInterface as tb_simpleSimpleArrayInterface;
use tb_simple::api::no_properties_interface::NoPropertiesInterfaceTrait as tb_simpleNoPropertiesInterfaceTrait;
use tb_simple::implementation::no_properties_interface::NoPropertiesInterface as tb_simpleNoPropertiesInterface;
use tb_simple::api::no_operations_interface::NoOperationsInterfaceTrait as tb_simpleNoOperationsInterfaceTrait;
use tb_simple::implementation::no_operations_interface::NoOperationsInterface as tb_simpleNoOperationsInterface;
use tb_simple::api::no_signals_interface::NoSignalsInterfaceTrait as tb_simpleNoSignalsInterfaceTrait;
use tb_simple::implementation::no_signals_interface::NoSignalsInterface as tb_simpleNoSignalsInterface;
use testbed1::api::struct_interface::StructInterfaceTrait as testbed1StructInterfaceTrait;
use testbed1::implementation::struct_interface::StructInterface as testbed1StructInterface;
use testbed1::api::struct_array_interface::StructArrayInterfaceTrait as testbed1StructArrayInterfaceTrait;
use testbed1::implementation::struct_array_interface::StructArrayInterface as testbed1StructArrayInterface;
use std::io;

fn main() {
    let mut _test_testbed2_many_param_interface = testbed2ManyParamInterface::default();
    let mut _test_testbed2_nested_struct1_interface = testbed2NestedStruct1Interface::default();
    let mut _test_testbed2_nested_struct2_interface = testbed2NestedStruct2Interface::default();
    let mut _test_testbed2_nested_struct3_interface = testbed2NestedStruct3Interface::default();
    let mut _test_tb_enum_enum_interface = tb_enumEnumInterface::default();
    let mut _test_tb_same1_same_struct1_interface = tb_same1SameStruct1Interface::default();
    let mut _test_tb_same1_same_struct2_interface = tb_same1SameStruct2Interface::default();
    let mut _test_tb_same1_same_enum1_interface = tb_same1SameEnum1Interface::default();
    let mut _test_tb_same1_same_enum2_interface = tb_same1SameEnum2Interface::default();
    let mut _test_tb_same2_same_struct1_interface = tb_same2SameStruct1Interface::default();
    let mut _test_tb_same2_same_struct2_interface = tb_same2SameStruct2Interface::default();
    let mut _test_tb_same2_same_enum1_interface = tb_same2SameEnum1Interface::default();
    let mut _test_tb_same2_same_enum2_interface = tb_same2SameEnum2Interface::default();
    let mut _test_tb_simple_void_interface = tb_simpleVoidInterface::default();
    let mut _test_tb_simple_simple_interface = tb_simpleSimpleInterface::default();
    let mut _test_tb_simple_simple_array_interface = tb_simpleSimpleArrayInterface::default();
    let mut _test_tb_simple_no_properties_interface = tb_simpleNoPropertiesInterface::default();
    let mut _test_tb_simple_no_operations_interface = tb_simpleNoOperationsInterface::default();
    let mut _test_tb_simple_no_signals_interface = tb_simpleNoSignalsInterface::default();
    let mut _test_testbed1_struct_interface = testbed1StructInterface::default();
    let mut _test_testbed1_struct_array_interface = testbed1StructArrayInterface::default();

    let mut cmd = String::new();
    println!("Enter command, or q to exit:");
    while cmd.trim() != "q" {
        cmd.clear();
        io::stdin().read_line(&mut cmd).unwrap();
    }
}
