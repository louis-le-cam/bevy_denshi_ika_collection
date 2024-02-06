/// Generates an unit `struct` and implement [`Plugin`] for it
///
/// [`bevy`] module needs to be in scope of the invocation
///
/// Using main as the plugin name will result in the creation
/// of the main function running the plugin
///
/// # Examples
/// ```rust
/// gen_plugins! {
///     pub(super) MySupberbPlugin;
///     /// [`App::add_plugins`]
///     plugins(SomeChildPlugin, SomeOtherChildPlugin);
///     /// [`App::register_type`]
///     reflect(SomeReflectType, SomeOtherReflectType);
///     /// [`App::add_systems`]
///     systems(Startup)(startup_system);
///     /// [`App::add_systems`] [`run_if`], [`in_state`]
///     systems(Update, SomeStates::SomeSate)(some_state_update);
///     /// [`App::init_resource`]
///     init_resources(SomeResource, SomeOtherResource);
///     /// [`App::init_non_send_resource`]
///     init_non_send_resources(SomeNonSendResource, SomeOtherNonSendResource);
///     /// [`App::init_schedules`]
///     init_schedules(SomeScheduleLabel, SomeOtherScheduleLabel);
///     /// [`App::add_state`]
///     states(SomeState, SomeOtherState);
///     /// [`App::add_event`]
///     events(SomeEvent, SomeOtherEvent)
///     /// [`App::insert_resource`]
///     resources(SomeResource::new(), SomeOtherResource::new());
///     /// [`App::insert_non_send_resource`]
///     non_send_resources(SomeNonSendResource::new(), SomeOtherNonSendResource::new());
///     /// [`App::set_runner`]
///     runner(my_runner);
///     /// [`App::init_asset`]
///     assets(SomeAsset, SomeOtherAsset);
///     /// [`App::init_asset_loader`]
///     asset_loaders(SomeAssetLoader, SomeOtherAssetLoader);
///     /// [`App::add_schedule`]
///     schedules(SomeSchedule, SomeOtherSchedule);
///     /// [`Plugin::build`]
///     build(|app| {});
///     /// [`Plugin::finish`]
///     finish(|app| {});
///     /// [`Plugin::cleanup`]
///     cleanup(|app| {});
///     #[cfg(feature = "dev")]
///     test_has(Or<(With<Character>, With<CharacterController>, With<CharacterSpeed>)>, (Character, CharacterController, CharacterSpeed));
/// }
/// ```
#[macro_export]
macro_rules! gen_plugin {

    {
        main;
        $($tail:tt)*
    } => {

        $crate::gen_plugin! {
            @internal { {} {} {} { , ; app } } $($tail)*
        }
    };

    {
        $vis:vis $name:ident;
        $($tail:tt)*
    } => {

        $crate::gen_plugin! {
            @internal { {} {} {} { $vis , $name ; app } } $($tail)*
        }
    };

    {
        @internal
        {
            { $($build:tt)* } { $($finish:tt)* } { $($cleanup:tt)* }
            { $vis:vis , $($name:ident)? ; $app:ident }
        }
        $(#[$attributes:meta])*
        plugins($($plugins:expr),* $(,)?);
        $($tail:tt)*
    } => {
        $crate::gen_plugin! {
            @internal
            {
                {
                    $($build)*
                    $(#[$attributes])*
                    $app.add_plugins(($($plugins),*));
                }
                { $($finish)* } { $($cleanup)* } { $vis , $($name)? ; $app }
            }
            $($tail)*
        }
    };

    {
        @internal
        {
            { $($build:tt)* } { $($finish:tt)* } { $($cleanup:tt)* }
            { $vis:vis , $($name:ident)? ; $app:ident }
        }
        $(#[$attributes:meta])*
        reflect($($types:ty),* $(,)?);
        $($tail:tt)*
    } => {
        $crate::gen_plugin! {
            @internal
            {
                {
                    $($build)*
                    $(#[$attributes])*
                    {$($app.register_type::<$types>();)*}
                }
                { $($finish)* } { $($cleanup)* } { $vis , $($name)? ; $app }
            }
            $($tail)*
        }
    };


    {
        @internal
        {
            { $($build:tt)* } { $($finish:tt)* } { $($cleanup:tt)* }
            { $vis:vis , $($name:ident)? ; $app:ident }
        }
        $(#[$attributes:meta])*
        systems($schedule:expr $(, $state:expr)?)($($systems:expr),* $(,)?);
        $($tail:tt)*
    } => {
        $crate::gen_plugin! {
            @internal
            {
                {
                    $($build)*
                    $(#[$attributes])*
                    $app.add_systems($schedule, ($($systems),*) $(.run_if(in_state($state)))?);
                }
                { $($finish)* } { $($cleanup)* } { $vis , $($name)? ; $app }
            }
            $($tail)*
        }
    };


    {
        @internal
        {
            { $($build:tt)* } { $($finish:tt)* } { $($cleanup:tt)* }
            { $vis:vis , $($name:ident)? ; $app:ident }
        }
        $(#[$attributes:meta])*
        init_resources($($resources:ty),* $(,)?);
        $($tail:tt)*
    } => {
        $crate::gen_plugin! {
            @internal
            {
                {
                    $($build)*
                    $(#[$attributes])*
                    {$($app.init_resource::<$resources>();)*};
                }
                { $($finish)* } { $($cleanup)* } { $vis , $($name)? ; $app }
            }
            $($tail)*
        }
    };


    {
        @internal
        {
            { $($build:tt)* } { $($finish:tt)* } { $($cleanup:tt)* }
            { $vis:vis , $($name:ident)? ; $app:ident }
        }
        $(#[$attributes:meta])*
        init_non_send_resources($($resources:ty),* $(,)?);
        $($tail:tt)*
    } => {
        $crate::gen_plugin! {
            @internal
            {
                {
                    $($build)*
                    $(#[$attributes])*
                    {$($app.init_non_send_resource::<$resources>();)*};
                }
                { $($finish)* } { $($cleanup)* } { $vis , $($name)? ; $app }
            }
            $($tail)*
        }
    };

    {
        @internal
        {
            { $($build:tt)* } { $($finish:tt)* } { $($cleanup:tt)* }
            { $vis:vis , $($name:ident)? ; $app:ident }
        }
        $(#[$attributes:meta])*
        init_schedules($($schedules:expr),* $(,)?);
        $($tail:tt)*
    } => {
        $crate::gen_plugin! {
            @internal
            {
                {
                    $($build)*
                    $(#[$attributes])*
                    {$($app.init_schedule($schedules);)*};
                }
                { $($finish)* } { $($cleanup)* } { $vis , $($name)? ; $app }
            }
            $($tail)*
        }
    };

    {
        @internal
        {
            { $($build:tt)* } { $($finish:tt)* } { $($cleanup:tt)* }
            { $vis:vis , $($name:ident)? ; $app:ident }
        }
        $(#[$attributes:meta])*
        states($($states:ty),* $(,)?);
        $($tail:tt)*
    } => {
        $crate::gen_plugin! {
            @internal
            {
                {
                    $($build)*
                    $(#[$attributes])*
                    {$($app.add_state::<$states>();)*};
                }
                { $($finish)* } { $($cleanup)* } { $vis , $($name)? ; $app }
            }
            $($tail)*
        }
    };

    {
        @internal
        {
            { $($build:tt)* } { $($finish:tt)* } { $($cleanup:tt)* }
            { $vis:vis , $($name:ident)? ; $app:ident }
        }
        $(#[$attributes:meta])*
        events($($events:ty),* $(,)?);
        $($tail:tt)*
    } => {
        $crate::gen_plugin! {
            @internal
            {
                {
                    $($build)*
                    $(#[$attributes])*
                    {$($app.add_event::<$events>();)*};
                }
                { $($finish)* } { $($cleanup)* } { $vis , $($name)? ; $app }
            }
            $($tail)*
        }
    };

    {
        @internal
        {
            { $($build:tt)* } { $($finish:tt)* } { $($cleanup:tt)* }
            { $vis:vis , $($name:ident)? ; $app:ident }
        }
        $(#[$attributes:meta])*
        resources($($resources:expr),* $(,)?);
        $($tail:tt)*
    } => {
        $crate::gen_plugin! {
            @internal
            {
                {
                    $($build)*
                    $(#[$attributes])*
                    {$($app.insert_resource($resources);)*};
                }
                { $($finish)* } { $($cleanup)* } { $vis , $($name)? ; $app }
            }
            $($tail)*
        }
    };

    {
        @internal
        {
            { $($build:tt)* } { $($finish:tt)* } { $($cleanup:tt)* }
            { $vis:vis , $($name:ident)? ; $app:ident }
        }
        $(#[$attributes:meta])*
        non_send_resources($($resources:expr),* $(,)?);
        $($tail:tt)*
    } => {
        $crate::gen_plugin! {
            @internal
            {
                {
                    $($build)*
                    $(#[$attributes])*
                    {$($app.insert_non_send_resource($resources);)*};
                }
                { $($finish)* } { $($cleanup)* } { $vis , $($name)? ; $app }
            }
            $($tail)*
        }
    };

    {
        @internal
        {
            { $($build:tt)* } { $($finish:tt)* } { $($cleanup:tt)* }
            { $vis:vis , $($name:ident)? ; $app:ident }
        }
        $(#[$attributes:meta])*
        runner($runner:expr $(,)?);
        $($tail:tt)*
    } => {
        $crate::gen_plugin! {
            @internal
            {
                {
                    $($build)*
                    $(#[$attributes])*
                    $app.set_runner($runner);
                }
                { $($finish)* } { $($cleanup)* } { $vis , $($name)? ; $app }
            }
            $($tail)*
        }
    };

    {
        @internal
        {
            { $($build:tt)* } { $($finish:tt)* } { $($cleanup:tt)* }
            { $vis:vis , $($name:ident)? ; $app:ident }
        }
        $(#[$attributes:meta])*
        assets($($assets:ty),* $(,)?);
        $($tail:tt)*
    } => {
        $crate::gen_plugin! {
            @internal
            {
                {
                    $($build)*
                    $(#[$attributes])*
                    {$($app.init_asset::<$assets>();)*};
                }
                { $($finish)* } { $($cleanup)* } { $vis , $($name)? ; $app }
            }
            $($tail)*
        }
    };

    {
        @internal
        {
            { $($build:tt)* } { $($finish:tt)* } { $($cleanup:tt)* }
            { $vis:vis , $($name:ident)? ; $app:ident }
        }
        $(#[$attributes:meta])*
        asset_loaders($($asset_loaders:ty),* $(,)?);
        $($tail:tt)*
    } => {
        $crate::gen_plugin! {
            @internal
            {
                {
                    $($build)*
                    $(#[$attributes])*
                    {$($app.init_asset_loader::<$asset_loaders>();)*};
                }
                { $($finish)* } { $($cleanup)* } { $vis , $($name)? ; $app }
            }
            $($tail)*
        }
    };

    {
        @internal
        {
            { $($build:tt)* } { $($finish:tt)* } { $($cleanup:tt)* }
            { $vis:vis , $($name:ident)? ; $app:ident }
        }
        $(#[$attributes:meta])*
        schedules($($schedules:expr),* $(,)?);
        $($tail:tt)*
    } => {
        $crate::gen_plugin! {
            @internal
            {
                {
                    $($build)*
                    $(#[$attributes])*
                    {$($app.add_schedule($schedules);)*};
                }
                { $($finish)* } { $($cleanup)* } { $vis , $($name)? ; $app }
            }
            $($tail)*
        }
    };

    {
        @internal
        {
            { $($build:tt)* } { $($finish:tt)* } { $($cleanup:tt)* }
            { $vis:vis , $($name:ident)? ; $app:ident }
        }
        $(#[$attributes:meta])*
        build($func:expr);
        $($tail:tt)*
    } => {
        $crate::gen_plugin! {
            @internal
            {
                {
                    $($build)*
                    $(#[$attributes])*
                    ($func)($app);
                }
                { $($finish)* } { $($cleanup)* } { $vis , $($name)? ; $app }
            }
            $($tail)*
        }
    };

    {
        @internal
        {
            { $($build:tt)* } { $($finish:tt)* } { $($cleanup:tt)* }
            { $vis:vis , $($name:ident)? ; $app:ident }
        }
        $(#[$attributes:meta])*
        finish($func:expr);
        $($tail:tt)*
    } => {
        $crate::gen_plugin! {
            @internal
            {
                { $($build)* }
                {
                    $($finish)*
                    $(#[$attributes])*
                    ($func)($app);
                }
                { $($cleanup)* } { $vis , $($name)? ; $app }
            }
            $($tail)*
        }
    };

    {
        @internal
        {
            { $($build:tt)* } { $($finish:tt)* } { $($cleanup:tt)* }
            { $vis:vis , $($name:ident)? ; $app:ident }
        }
        $(#[$attributes:meta])*
        cleanup($func:expr);
        $($tail:tt)*
    } => {
        $crate::gen_plugin! {
            @internal
            {
                { $($build)* } { $($finish)* }
                {
                    $($cleanup)*
                    $(#[$attributes])*
                    ($func)($app);
                }
                { $vis , $($name)? ; $app }
            }
            $($tail)*
        }
    };

    {
        @internal
        {
            { $($build:tt)* } { $($finish:tt)* } { $($cleanup:tt)* }
            { $vis:vis , $($name:ident)? ; $app:ident }
        }
        $(#[$attributes:meta])*
        test_has($filter:ty, ($($required:ty),+));
        $($tail:tt)*
    } => {
        $crate::gen_plugin! {
            @internal
            {
                {
                    $($build)*
                    $(#[$attributes])*
                    $app.add_systems(
                        Update,
                        |query: Query<(DebugName, ($(Has<$required>),+)), $filter>| {
                            for (debug_name, elements) in &query {
                                let elements = Into::<[bool; $((1, std::marker::PhantomData::<$required>).0 +)+ 0]>::into(elements);

                                if elements.into_iter().all(|has| has) {
                                    continue;
                                };


                                bevy::log::error!(
                                    "{}",
                                    format!(
                                        "Invalid entity {:?} {}",
                                        debug_name,
                                        [$((stringify!($required))),*]
                                            .iter()
                                            .enumerate()
                                            .map(|(i, ty)| format!("\n\t{}: {}", ty, elements[i]))
                                            .collect::<Vec<String>>()
                                            .join("")
                                    )
                                );
                            }
                            // for element in &query {
                            //     ![$(stringify!($required)),*].iter().enumerate().all(|(n, has)|);
                            //     if $(!$required)||* {
                            //         bevy::log::error!(
                            //             "{}",
                            //             format!(
                            //                 "Invalid entity {:?} {}",
                            //                 debug_name,
                            //                 [$(($required, stringify!($reqired)))*]
                            //                     .iter()
                            //                     .map(|(has, ty)| format!("\n\t{}: {}", ty, has))
                            //                     .join("")
                            //             )
                            //         );
                            //     }
                            // }
                        }
                    );
                }
                { $($finish)* } { $($cleanup)* }
                { $vis , $($name)? ; $app }
            }
            $($tail)*
        }
    };

    {
        @internal
        {
            { $($build:tt)* } { $($finish:tt)* } { $($cleanup:tt)* }
            { $vis:vis , ; $app:ident }
        }
    } => {
        fn main() {
            struct MainPlugin;
            $crate::gen_plugin! {
                @internal_impl
                {
                    { $($build)* } { $($finish)* } { $($cleanup)* }
                    { MainPlugin $app }
                }
            }

            bevy::prelude::App::new().add_plugins(MainPlugin).run();
        }

    };

    {
        @internal
        {
            { $($build:tt)* } { $($finish:tt)* } { $($cleanup:tt)* }
            { $vis:vis , $name:ident ; $app:ident }
        }
    } => {
        $vis struct $name;

        $crate::gen_plugin! {
            @internal_impl
            {
                { $($build)* } { $($finish)* } { $($cleanup)* }
                { $name $app }
            }
        }
    };

    {
        @internal_impl
        {
            { $($build:tt)* } { $($finish:tt)* } { $($cleanup:tt)* }
            { $name:ident $app:ident }
        }
    } => {
        impl bevy::prelude::Plugin for $name {
            fn build(&self, $app: &mut bevy::prelude::App) {
                use bevy::prelude::*;

                $($build)*
            }

            fn finish(&self, $app: &mut bevy::prelude::App) {
                use bevy::prelude::*;

                $($finish)*
            }

            fn cleanup(&self, $app: &mut bevy::prelude::App) {
                use bevy::prelude::*;

                $($cleanup)*
            }
        }
    };
}
