macro_rules! spawn {
    ($to_spawn:expr) => {{
        let spawnable: Spawnable = Box::new($to_spawn.into_future().map(|_| ()).map_err(|_| ()));
        crate::runtime::BOOMSLANG.spawn(spawnable)
    }};
}

macro_rules! for_match {
    ($eenum:ident : $type:ident [ $($name:ident),* ] $out:ident $e:block ) => {
        match $eenum {
            $( $type::$name($out) => { $e }, )*
        }
    };
    ($eenum:ident : $type:ident [ $($name:ident),* ] |$out:ident| ( $( $t:tt )* ) ) => {
        for_match!($eenum : $type [ $( $name ),* ] $out { $( $t )* })
    };
}

macro_rules! impl_module {
    ($module_name:ident, $module_trait:ident :
        { $($name:ident, $maker:ident =>
            { $($argname:ident : $argtype:ty),* } )+
        } => {
            $( $fn:ident ( $( $arg:ident : $type:ty ),* ) -> $ret:ty )*
        }
    ) => {

        pub trait $module_trait {
            $( fn $fn(&self, $( $arg : $type )* ) -> $ret )* ;
        }

        #[derive(Debug, Clone)]
        pub enum $module_name {
            $($name(Arc<$name>),)*
        }

        impl $module_name {
            #[allow(dead_code)]
            pub fn get_name(&self) -> String {
                match self {
                    $($module_name::$name(inner) => inner.name.to_string(),)*
                }
            }

            $(
                pub fn $maker(name: String, $($argname: $argtype , )*) -> Self {
                    let (sender, receiver) = unbounded_channel();
                    $module_name::$name(Arc::new($name{
                        name,
                        receiver: Mutex::new(Some(receiver)),
                        sender,
                        $($argname , )*
                    }))
                }
            )*
        }

        macro_rules! unroll {
            ($n:ident, $method:ident, ) => {
                match $n {
                    $( $module_name::$name(inner) => inner.$method() , )*
                }
            };
            ($n:ident, $method:ident, $i:ident, ) => {
                match $n {
                    $( $module_name::$name(inner) => inner.$method($i) , )*
                }
            };
        }

        impl $module_trait for $module_name {
            $(
                fn $fn (&self, $( $arg : $type )* ) -> $ret {
                    unroll!(self, $fn, $( $arg , )* )
                }
            )*
        }

        impl $module_name {
            #[allow(dead_code)]
            pub fn send(&self, message: Message) {
                match self {
                    $( $module_name::$name(inner) => inner.sender.clone().try_send(message).unwrap_or_default(), )*
                };
            }
        }

        $(
            impl From<$name> for $module_name {
                fn from(m: $name) -> Self {
                    $module_name::$name(Arc::new(m))
                }
            }
        )*

        $(
            #[derive(Debug)]
            pub struct $name{
                pub name: String,
                pub sender: UnboundedSender<Message>,
                pub receiver: Mutex<Option<UnboundedReceiver<Message>>>,
                $(pub $argname: $argtype , )*
            }
        )*
    };
}
