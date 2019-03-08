macro_rules! script_prefab (
    {
    $class_name:ident {
        [Vars]
        $($var:ident: $var_type:ty = $var_default:expr;)*
        [Components]
        $($comp:ident: $comp_type:ty = $comp_default:expr;)*
        [Behaviour]
        fn new($new_vars:ident, $new_world:ident) $new_function:block
        fn update($update_vars:ident, $update_world:ident) $update_function:block
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
            pub fn init(w: &mut GameState) {
                w.register_component::<$class_name>();
            }
            pub fn new($new_world: &GameState) -> Entity {
                let e = $new_world.create_entity();
                $new_world.insert(e, $class_name{$($var: $var_default),*});
                $(
                    $new_world.insert(e, $comp_default);
                )*
                $new_world.run(|($new_vars, $($comp),*): (&mut $class_name, $(&mut $comp_type),*)| {
                    $new_function
                });
                e
            }
            pub fn update($update_world: &GameState) {
                $update_world.run(|($update_vars, $($comp),*): (&mut $class_name, $(&mut $comp_type),*)| {
                    $update_function
                });
            }
        }
    }
);
