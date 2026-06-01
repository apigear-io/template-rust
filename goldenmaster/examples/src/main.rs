//! Local example: use the generated default implementations directly, in-process.
//! Each interface is instantiated and exercised (operations, properties, signals)
//! without any IPC. See `src/bin/{olink,mqtt,nats}_{server,client}.rs` for the IPC
//! examples.
#![allow(unused_imports, unused_variables)]

#[tokio::main(flavor = "multi_thread")]
async fn main() {
    println!("ApiGear Rust SDK — local implementation example");
    println!("===============================================");
    {
        use testbed2::api::many_param_interface::ManyParamInterfaceTrait;
        use testbed2::api::many_param_interface::ManyParamInterfaceTraitAsync;
        use testbed2::implementation::many_param_interface::ManyParamInterface;
        #[allow(unused_imports)]
        use testbed2::api::data_structs::*;
        println!("\n== testbed2.ManyParamInterface ==");
        let object = ManyParamInterface::default();
        // call the first operation through the async wrapper
        let _ = object.func1_async(Default::default()).await;
        println!("  called func1()");
        let value: i32 = 42i32;
        object.set_prop1(value);
        println!("  prop1 = {:?}", object.prop1());
        let value: i32 = 42i32;
        object.set_prop2(value);
        println!("  prop2 = {:?}", object.prop2());
        let value: i32 = 42i32;
        object.set_prop3(value);
        println!("  prop3 = {:?}", object.prop3());
        let value: i32 = 42i32;
        object.set_prop4(value);
        println!("  prop4 = {:?}", object.prop4());
        let _publisher = object.publisher();
        println!("  4 signal(s) available via publisher()");
    }
    {
        use testbed2::api::nested_struct1_interface::NestedStruct1InterfaceTrait;
        use testbed2::api::nested_struct1_interface::NestedStruct1InterfaceTraitAsync;
        use testbed2::implementation::nested_struct1_interface::NestedStruct1Interface;
        #[allow(unused_imports)]
        use testbed2::api::data_structs::*;
        println!("\n== testbed2.NestedStruct1Interface ==");
        let object = NestedStruct1Interface::default();
        // call the first operation through the async wrapper
        let _ = object.func_no_return_value_async(&Default::default()).await;
        println!("  called func_no_return_value()");
        let value: NestedStruct1 = Default::default();
        object.set_prop1(&value.clone());
        println!("  prop1 = {:?}", object.prop1());
        let _publisher = object.publisher();
        println!("  1 signal(s) available via publisher()");
    }
    {
        use testbed2::api::nested_struct2_interface::NestedStruct2InterfaceTrait;
        use testbed2::api::nested_struct2_interface::NestedStruct2InterfaceTraitAsync;
        use testbed2::implementation::nested_struct2_interface::NestedStruct2Interface;
        #[allow(unused_imports)]
        use testbed2::api::data_structs::*;
        println!("\n== testbed2.NestedStruct2Interface ==");
        let object = NestedStruct2Interface::default();
        // call the first operation through the async wrapper
        let _ = object.func1_async(&Default::default()).await;
        println!("  called func1()");
        let value: NestedStruct1 = Default::default();
        object.set_prop1(&value.clone());
        println!("  prop1 = {:?}", object.prop1());
        let value: NestedStruct2 = Default::default();
        object.set_prop2(&value.clone());
        println!("  prop2 = {:?}", object.prop2());
        let _publisher = object.publisher();
        println!("  2 signal(s) available via publisher()");
    }
    {
        use testbed2::api::nested_struct3_interface::NestedStruct3InterfaceTrait;
        use testbed2::api::nested_struct3_interface::NestedStruct3InterfaceTraitAsync;
        use testbed2::implementation::nested_struct3_interface::NestedStruct3Interface;
        #[allow(unused_imports)]
        use testbed2::api::data_structs::*;
        println!("\n== testbed2.NestedStruct3Interface ==");
        let object = NestedStruct3Interface::default();
        // call the first operation through the async wrapper
        let _ = object.func1_async(&Default::default()).await;
        println!("  called func1()");
        let value: NestedStruct1 = Default::default();
        object.set_prop1(&value.clone());
        println!("  prop1 = {:?}", object.prop1());
        let value: NestedStruct2 = Default::default();
        object.set_prop2(&value.clone());
        println!("  prop2 = {:?}", object.prop2());
        let value: NestedStruct3 = Default::default();
        object.set_prop3(&value.clone());
        println!("  prop3 = {:?}", object.prop3());
        let _publisher = object.publisher();
        println!("  3 signal(s) available via publisher()");
    }
    {
        use tb_enum::api::enum_interface::EnumInterfaceTrait;
        use tb_enum::api::enum_interface::EnumInterfaceTraitAsync;
        use tb_enum::implementation::enum_interface::EnumInterface;
        #[allow(unused_imports)]
        use tb_enum::api::data_structs::*;
        println!("\n== tb.enum.EnumInterface ==");
        let object = EnumInterface::default();
        // call the first operation through the async wrapper
        let _ = object.func0_async(Default::default()).await;
        println!("  called func0()");
        let value: Enum0Enum = Default::default();
        object.set_prop0(value);
        println!("  prop0 = {:?}", object.prop0());
        let value: Enum1Enum = Default::default();
        object.set_prop1(value);
        println!("  prop1 = {:?}", object.prop1());
        let value: Enum2Enum = Default::default();
        object.set_prop2(value);
        println!("  prop2 = {:?}", object.prop2());
        let value: Enum3Enum = Default::default();
        object.set_prop3(value);
        println!("  prop3 = {:?}", object.prop3());
        let _publisher = object.publisher();
        println!("  4 signal(s) available via publisher()");
    }
    {
        use tb_same1::api::same_struct1_interface::SameStruct1InterfaceTrait;
        use tb_same1::api::same_struct1_interface::SameStruct1InterfaceTraitAsync;
        use tb_same1::implementation::same_struct1_interface::SameStruct1Interface;
        #[allow(unused_imports)]
        use tb_same1::api::data_structs::*;
        println!("\n== tb.same1.SameStruct1Interface ==");
        let object = SameStruct1Interface::default();
        // call the first operation through the async wrapper
        let _ = object.func1_async(&Default::default()).await;
        println!("  called func1()");
        let value: Struct1 = Default::default();
        object.set_prop1(&value.clone());
        println!("  prop1 = {:?}", object.prop1());
        let _publisher = object.publisher();
        println!("  1 signal(s) available via publisher()");
    }
    {
        use tb_same1::api::same_struct2_interface::SameStruct2InterfaceTrait;
        use tb_same1::api::same_struct2_interface::SameStruct2InterfaceTraitAsync;
        use tb_same1::implementation::same_struct2_interface::SameStruct2Interface;
        #[allow(unused_imports)]
        use tb_same1::api::data_structs::*;
        println!("\n== tb.same1.SameStruct2Interface ==");
        let object = SameStruct2Interface::default();
        // call the first operation through the async wrapper
        let _ = object.func1_async(&Default::default()).await;
        println!("  called func1()");
        let value: Struct2 = Default::default();
        object.set_prop1(&value.clone());
        println!("  prop1 = {:?}", object.prop1());
        let value: Struct2 = Default::default();
        object.set_prop2(&value.clone());
        println!("  prop2 = {:?}", object.prop2());
        let _publisher = object.publisher();
        println!("  2 signal(s) available via publisher()");
    }
    {
        use tb_same1::api::same_enum1_interface::SameEnum1InterfaceTrait;
        use tb_same1::api::same_enum1_interface::SameEnum1InterfaceTraitAsync;
        use tb_same1::implementation::same_enum1_interface::SameEnum1Interface;
        #[allow(unused_imports)]
        use tb_same1::api::data_structs::*;
        println!("\n== tb.same1.SameEnum1Interface ==");
        let object = SameEnum1Interface::default();
        // call the first operation through the async wrapper
        let _ = object.func1_async(Default::default()).await;
        println!("  called func1()");
        let value: Enum1Enum = Default::default();
        object.set_prop1(value);
        println!("  prop1 = {:?}", object.prop1());
        let _publisher = object.publisher();
        println!("  1 signal(s) available via publisher()");
    }
    {
        use tb_same1::api::same_enum2_interface::SameEnum2InterfaceTrait;
        use tb_same1::api::same_enum2_interface::SameEnum2InterfaceTraitAsync;
        use tb_same1::implementation::same_enum2_interface::SameEnum2Interface;
        #[allow(unused_imports)]
        use tb_same1::api::data_structs::*;
        println!("\n== tb.same1.SameEnum2Interface ==");
        let object = SameEnum2Interface::default();
        // call the first operation through the async wrapper
        let _ = object.func1_async(Default::default()).await;
        println!("  called func1()");
        let value: Enum1Enum = Default::default();
        object.set_prop1(value);
        println!("  prop1 = {:?}", object.prop1());
        let value: Enum2Enum = Default::default();
        object.set_prop2(value);
        println!("  prop2 = {:?}", object.prop2());
        let _publisher = object.publisher();
        println!("  2 signal(s) available via publisher()");
    }
    {
        use tb_same2::api::same_struct1_interface::SameStruct1InterfaceTrait;
        use tb_same2::api::same_struct1_interface::SameStruct1InterfaceTraitAsync;
        use tb_same2::implementation::same_struct1_interface::SameStruct1Interface;
        #[allow(unused_imports)]
        use tb_same2::api::data_structs::*;
        println!("\n== tb.same2.SameStruct1Interface ==");
        let object = SameStruct1Interface::default();
        // call the first operation through the async wrapper
        let _ = object.func1_async(&Default::default()).await;
        println!("  called func1()");
        let value: Struct1 = Default::default();
        object.set_prop1(&value.clone());
        println!("  prop1 = {:?}", object.prop1());
        let _publisher = object.publisher();
        println!("  1 signal(s) available via publisher()");
    }
    {
        use tb_same2::api::same_struct2_interface::SameStruct2InterfaceTrait;
        use tb_same2::api::same_struct2_interface::SameStruct2InterfaceTraitAsync;
        use tb_same2::implementation::same_struct2_interface::SameStruct2Interface;
        #[allow(unused_imports)]
        use tb_same2::api::data_structs::*;
        println!("\n== tb.same2.SameStruct2Interface ==");
        let object = SameStruct2Interface::default();
        // call the first operation through the async wrapper
        let _ = object.func1_async(&Default::default()).await;
        println!("  called func1()");
        let value: Struct2 = Default::default();
        object.set_prop1(&value.clone());
        println!("  prop1 = {:?}", object.prop1());
        let value: Struct2 = Default::default();
        object.set_prop2(&value.clone());
        println!("  prop2 = {:?}", object.prop2());
        let _publisher = object.publisher();
        println!("  2 signal(s) available via publisher()");
    }
    {
        use tb_same2::api::same_enum1_interface::SameEnum1InterfaceTrait;
        use tb_same2::api::same_enum1_interface::SameEnum1InterfaceTraitAsync;
        use tb_same2::implementation::same_enum1_interface::SameEnum1Interface;
        #[allow(unused_imports)]
        use tb_same2::api::data_structs::*;
        println!("\n== tb.same2.SameEnum1Interface ==");
        let object = SameEnum1Interface::default();
        // call the first operation through the async wrapper
        let _ = object.func1_async(Default::default()).await;
        println!("  called func1()");
        let value: Enum1Enum = Default::default();
        object.set_prop1(value);
        println!("  prop1 = {:?}", object.prop1());
        let _publisher = object.publisher();
        println!("  1 signal(s) available via publisher()");
    }
    {
        use tb_same2::api::same_enum2_interface::SameEnum2InterfaceTrait;
        use tb_same2::api::same_enum2_interface::SameEnum2InterfaceTraitAsync;
        use tb_same2::implementation::same_enum2_interface::SameEnum2Interface;
        #[allow(unused_imports)]
        use tb_same2::api::data_structs::*;
        println!("\n== tb.same2.SameEnum2Interface ==");
        let object = SameEnum2Interface::default();
        // call the first operation through the async wrapper
        let _ = object.func1_async(Default::default()).await;
        println!("  called func1()");
        let value: Enum1Enum = Default::default();
        object.set_prop1(value);
        println!("  prop1 = {:?}", object.prop1());
        let value: Enum2Enum = Default::default();
        object.set_prop2(value);
        println!("  prop2 = {:?}", object.prop2());
        let _publisher = object.publisher();
        println!("  2 signal(s) available via publisher()");
    }
    {
        use tb_simple::api::void_interface::VoidInterfaceTrait;
        use tb_simple::api::void_interface::VoidInterfaceTraitAsync;
        use tb_simple::implementation::void_interface::VoidInterface;
        println!("\n== tb.simple.VoidInterface ==");
        let object = VoidInterface::default();
        // call the first operation through the async wrapper
        let _ = object.func_void_async().await;
        println!("  called func_void()");
        let _publisher = object.publisher();
        println!("  1 signal(s) available via publisher()");
    }
    {
        use tb_simple::api::simple_interface::SimpleInterfaceTrait;
        use tb_simple::api::simple_interface::SimpleInterfaceTraitAsync;
        use tb_simple::implementation::simple_interface::SimpleInterface;
        println!("\n== tb.simple.SimpleInterface ==");
        let object = SimpleInterface::default();
        // call the first operation through the async wrapper
        let _ = object.func_no_return_value_async(Default::default()).await;
        println!("  called func_no_return_value()");
        let value: bool = true;
        object.set_prop_bool(value);
        println!("  prop_bool = {:?}", object.prop_bool());
        let value: i32 = 42i32;
        object.set_prop_int(value);
        println!("  prop_int = {:?}", object.prop_int());
        let value: i32 = 42i32;
        object.set_prop_int32(value);
        println!("  prop_int32 = {:?}", object.prop_int32());
        let value: i64 = 42i64;
        object.set_prop_int64(value);
        println!("  prop_int64 = {:?}", object.prop_int64());
        let value: f32 = 4.2f32;
        object.set_prop_float(value);
        println!("  prop_float = {:?}", object.prop_float());
        let value: f32 = 4.2f32;
        object.set_prop_float32(value);
        println!("  prop_float32 = {:?}", object.prop_float32());
        let value: f64 = 4.2f64;
        object.set_prop_float64(value);
        println!("  prop_float64 = {:?}", object.prop_float64());
        let value: String = String::from("hello");
        object.set_prop_string(value.as_str());
        println!("  prop_string = {:?}", object.prop_string());
        let _publisher = object.publisher();
        println!("  8 signal(s) available via publisher()");
    }
    {
        use tb_simple::api::simple_array_interface::SimpleArrayInterfaceTrait;
        use tb_simple::api::simple_array_interface::SimpleArrayInterfaceTraitAsync;
        use tb_simple::implementation::simple_array_interface::SimpleArrayInterface;
        println!("\n== tb.simple.SimpleArrayInterface ==");
        let object = SimpleArrayInterface::default();
        // call the first operation through the async wrapper
        let _ = object.func_bool_async(Default::default()).await;
        println!("  called func_bool()");
        let value: Vec<bool> = vec![Default::default()];
        object.set_prop_bool(value.as_slice());
        println!("  prop_bool = {:?}", object.prop_bool());
        let value: Vec<i32> = vec![Default::default()];
        object.set_prop_int(value.as_slice());
        println!("  prop_int = {:?}", object.prop_int());
        let value: Vec<i32> = vec![Default::default()];
        object.set_prop_int32(value.as_slice());
        println!("  prop_int32 = {:?}", object.prop_int32());
        let value: Vec<i64> = vec![Default::default()];
        object.set_prop_int64(value.as_slice());
        println!("  prop_int64 = {:?}", object.prop_int64());
        let value: Vec<f32> = vec![Default::default()];
        object.set_prop_float(value.as_slice());
        println!("  prop_float = {:?}", object.prop_float());
        let value: Vec<f32> = vec![Default::default()];
        object.set_prop_float32(value.as_slice());
        println!("  prop_float32 = {:?}", object.prop_float32());
        let value: Vec<f64> = vec![Default::default()];
        object.set_prop_float64(value.as_slice());
        println!("  prop_float64 = {:?}", object.prop_float64());
        let value: Vec<String> = vec![Default::default()];
        object.set_prop_string(value.as_slice());
        println!("  prop_string = {:?}", object.prop_string());
        let _publisher = object.publisher();
        println!("  8 signal(s) available via publisher()");
    }
    {
        use tb_simple::api::no_properties_interface::NoPropertiesInterfaceTrait;
        use tb_simple::api::no_properties_interface::NoPropertiesInterfaceTraitAsync;
        use tb_simple::implementation::no_properties_interface::NoPropertiesInterface;
        println!("\n== tb.simple.NoPropertiesInterface ==");
        let object = NoPropertiesInterface::default();
        // call the first operation through the async wrapper
        let _ = object.func_void_async().await;
        println!("  called func_void()");
        let _publisher = object.publisher();
        println!("  2 signal(s) available via publisher()");
    }
    {
        use tb_simple::api::no_operations_interface::NoOperationsInterfaceTrait;
        use tb_simple::implementation::no_operations_interface::NoOperationsInterface;
        println!("\n== tb.simple.NoOperationsInterface ==");
        let object = NoOperationsInterface::default();
        let value: bool = true;
        object.set_prop_bool(value);
        println!("  prop_bool = {:?}", object.prop_bool());
        let value: i32 = 42i32;
        object.set_prop_int(value);
        println!("  prop_int = {:?}", object.prop_int());
        let _publisher = object.publisher();
        println!("  2 signal(s) available via publisher()");
    }
    {
        use tb_simple::api::no_signals_interface::NoSignalsInterfaceTrait;
        use tb_simple::api::no_signals_interface::NoSignalsInterfaceTraitAsync;
        use tb_simple::implementation::no_signals_interface::NoSignalsInterface;
        println!("\n== tb.simple.NoSignalsInterface ==");
        let object = NoSignalsInterface::default();
        // call the first operation through the async wrapper
        let _ = object.func_void_async().await;
        println!("  called func_void()");
        let value: bool = true;
        object.set_prop_bool(value);
        println!("  prop_bool = {:?}", object.prop_bool());
        let value: i32 = 42i32;
        object.set_prop_int(value);
        println!("  prop_int = {:?}", object.prop_int());
    }
    {
        use tb_simple::api::empty_interface::EmptyInterfaceTrait;
        use tb_simple::implementation::empty_interface::EmptyInterface;
        println!("\n== tb.simple.EmptyInterface ==");
        let object = EmptyInterface::default();
    }
    {
        use testbed1::api::struct_interface::StructInterfaceTrait;
        use testbed1::api::struct_interface::StructInterfaceTraitAsync;
        use testbed1::implementation::struct_interface::StructInterface;
        #[allow(unused_imports)]
        use testbed1::api::data_structs::*;
        println!("\n== testbed1.StructInterface ==");
        let object = StructInterface::default();
        // call the first operation through the async wrapper
        let _ = object.func_bool_async(&Default::default()).await;
        println!("  called func_bool()");
        let value: StructBool = Default::default();
        object.set_prop_bool(&value.clone());
        println!("  prop_bool = {:?}", object.prop_bool());
        let value: StructInt = Default::default();
        object.set_prop_int(&value.clone());
        println!("  prop_int = {:?}", object.prop_int());
        let value: StructFloat = Default::default();
        object.set_prop_float(&value.clone());
        println!("  prop_float = {:?}", object.prop_float());
        let value: StructString = Default::default();
        object.set_prop_string(&value.clone());
        println!("  prop_string = {:?}", object.prop_string());
        let _publisher = object.publisher();
        println!("  4 signal(s) available via publisher()");
    }
    {
        use testbed1::api::struct_array_interface::StructArrayInterfaceTrait;
        use testbed1::api::struct_array_interface::StructArrayInterfaceTraitAsync;
        use testbed1::implementation::struct_array_interface::StructArrayInterface;
        #[allow(unused_imports)]
        use testbed1::api::data_structs::*;
        println!("\n== testbed1.StructArrayInterface ==");
        let object = StructArrayInterface::default();
        // call the first operation through the async wrapper
        let _ = object.func_bool_async(Default::default()).await;
        println!("  called func_bool()");
        let value: Vec<StructBool> = vec![Default::default()];
        object.set_prop_bool(value.as_slice());
        println!("  prop_bool = {:?}", object.prop_bool());
        let value: Vec<StructInt> = vec![Default::default()];
        object.set_prop_int(value.as_slice());
        println!("  prop_int = {:?}", object.prop_int());
        let value: Vec<StructFloat> = vec![Default::default()];
        object.set_prop_float(value.as_slice());
        println!("  prop_float = {:?}", object.prop_float());
        let value: Vec<StructString> = vec![Default::default()];
        object.set_prop_string(value.as_slice());
        println!("  prop_string = {:?}", object.prop_string());
        let value: Vec<Enum0Enum> = vec![Default::default()];
        object.set_prop_enum(value.as_slice());
        println!("  prop_enum = {:?}", object.prop_enum());
        let _publisher = object.publisher();
        println!("  5 signal(s) available via publisher()");
    }
    {
        use testbed1::api::struct_array2_interface::StructArray2InterfaceTrait;
        use testbed1::api::struct_array2_interface::StructArray2InterfaceTraitAsync;
        use testbed1::implementation::struct_array2_interface::StructArray2Interface;
        #[allow(unused_imports)]
        use testbed1::api::data_structs::*;
        println!("\n== testbed1.StructArray2Interface ==");
        let object = StructArray2Interface::default();
        // call the first operation through the async wrapper
        let _ = object.func_bool_async(&Default::default()).await;
        println!("  called func_bool()");
        let value: StructBoolWithArray = Default::default();
        object.set_prop_bool(&value.clone());
        println!("  prop_bool = {:?}", object.prop_bool());
        let value: StructIntWithArray = Default::default();
        object.set_prop_int(&value.clone());
        println!("  prop_int = {:?}", object.prop_int());
        let value: StructFloatWithArray = Default::default();
        object.set_prop_float(&value.clone());
        println!("  prop_float = {:?}", object.prop_float());
        let value: StructStringWithArray = Default::default();
        object.set_prop_string(&value.clone());
        println!("  prop_string = {:?}", object.prop_string());
        let value: StructEnumWithArray = Default::default();
        object.set_prop_enum(&value.clone());
        println!("  prop_enum = {:?}", object.prop_enum());
        let _publisher = object.publisher();
        println!("  4 signal(s) available via publisher()");
    }
    {
        use tb_names::api::nam_es::NamEsTrait;
        use tb_names::api::nam_es::NamEsTraitAsync;
        use tb_names::implementation::nam_es::NamEs;
        #[allow(unused_imports)]
        use tb_names::api::data_structs::*;
        println!("\n== tb.names.NamEs ==");
        let object = NamEs::default();
        // call the first operation through the async wrapper
        let _ = object.some_function_async(Default::default()).await;
        println!("  called some_function()");
        let value: bool = true;
        object.set_switch(value);
        println!("  switch = {:?}", object.switch());
        let value: i32 = 42i32;
        object.set_some_property(value);
        println!("  some_property = {:?}", object.some_property());
        let value: i32 = 42i32;
        object.set_some_poperty2(value);
        println!("  some_poperty2 = {:?}", object.some_poperty2());
        let value: Enum_With_Under_scoresEnum = Default::default();
        object.set_enum_property(value);
        println!("  enum_property = {:?}", object.enum_property());
        let _publisher = object.publisher();
        println!("  2 signal(s) available via publisher()");
    }

    println!("\nAll interfaces exercised locally.");
}
