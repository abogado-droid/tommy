use tommy::*;

#[test]
fn example() {
    macro_rules! config_table {
        ($nme:ident { $($fld:ident : $typ:ty),* $(,)? }) => {
            #[derive(Debug)]
            #[allow(unused)]
            struct $nme {
            $($fld: $typ),*
            }
            from_table_struct!($nme {
            $($fld: $typ),*
            });
        };
    }

    config_table!(Cursor {
        blink: bool,
        blink_duration: i32,
    });

    config_table!(Window {
        title: String,
        width: f64,
        height: f64,
    });

    config_table!(Icons {
        entry: char,
        exit: char,
        controls: char,
    });

    let parsed_user = ParseConfig::from_file("test.toml".to_string()).unwrap();
    let parsed_fabk = ParseConfig::from_file("fallback.toml".to_string()).unwrap();

    macro_rules! load_conf {
        ($var:ident : $ty:ty) => {
            let $var: $ty = parsed_user
            .table(stringify!($ty).to_lowercase().as_str())
            .or_else(|| {
            parsed_fabk.table(stringify!($ty).to_lowercase().as_str())
            })
            .unwrap();
        };
    }

    load_conf!(cursor_conf: Cursor);
    load_conf!(window_conf: Window);
    load_conf!(icons_conf: Icons);

    println!("{:#?}", cursor_conf);
    println!("{:#?}", window_conf);
    println!("{:#?}", icons_conf);
}
