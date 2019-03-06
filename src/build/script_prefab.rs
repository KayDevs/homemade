macro_rules! script_prefab (
    {
    $class_name:ident {
        [Vars]
        $($var:ident: $var_type:ty = $var_default:expr;)*
        [Components]
        $($comp:ident: $comp_type:ty = $comp_default:expr;)*
        [Behaviour]
        fn new($new_vars:ident) $new_function:block
        fn update($update_vars:ident) $update_function:block
    }} => {
        #[derive(Clone)]
        pub struct $class_name {
            $(pub $var: $var_type),*
        }
        impl Component for $class_name {
            type Storage = VecStorage<Self>;
        }
        #[allow(unused)]
        impl $class_name {
            pub fn new(w: &mut GameState) -> Entity {
                w.register_component::<$class_name>();
                let e = w.create_entity();
                w.insert(e, $class_name{$($var: $var_default),*});
                $(
                    w.insert(e, $comp_default);
                )*
                w.run(|($new_vars, $($comp),*): (&mut $class_name, $(&mut $comp_type),*)| {
                    $new_function
                });
                e
            }
            pub fn update(w: &GameState) {
                w.run(|($update_vars, $($comp),*): (&mut $class_name, $(&mut $comp_type),*)| {
                    $update_function
                });
            }
        }
    }
);
