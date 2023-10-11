use regex::Regex;
use rutie::{
    class, methods, wrappable_struct, AnyObject, Array, Boolean, Class, Object, RString, VM,
};

wrappable_struct!(Regex, RegexWrapper, REGEX_WRAPPER);

class!(Regox);

methods!(
    Regox,
    rtself,
    fn regox_regex_new(pattern: RString) -> AnyObject {
        let pattern = pattern.map_err(|e| VM::raise_ex(e)).unwrap();

        let regex = Regex::new(pattern.to_str())
            .map_err(|_| VM::raise(Class::from_existing("StandardError"), "Compile Error"))
            .unwrap();

        Class::from_existing("Regox").wrap_data(regex, &*REGEX_WRAPPER)
    },
    fn regox_regex_is_match(input: RString) -> Boolean {
        let regex = rtself.get_data(&*REGEX_WRAPPER);
        let haystack = input.map_err(|e| VM::raise_ex(e)).unwrap();
        Boolean::new(regex.is_match(haystack.to_str_unchecked()))
    },
    fn regox_regex_scan(input: RString) -> Array {
        let regex = rtself.get_data(&*REGEX_WRAPPER);
        let haystack = input.map_err(|e| VM::raise_ex(e)).unwrap();

        let mut result = Array::new();

        for captures in regex.captures_iter(haystack.to_str()) {
            if regex.captures_len() == 1 {
                result.push(RString::new_usascii_unchecked(
                    captures.get(0).unwrap().as_str(),
                ));
            } else {
                let mut c = Array::with_capacity(regex.captures_len());

                for capture in captures.iter().skip(1) {
                    if let Some(capture) = capture {
                        c.push(RString::new_usascii_unchecked(capture.as_str()));
                    } else {
                        c.push(RString::new_usascii_unchecked(""));
                    }
                }

                result.push(c);
            }
        }

        result
    }
);

#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn Init_regox_regex() {
    let data_class = Class::from_existing("Object");
    Class::new("Regox", Some(&data_class)).define(|klass| {
        klass.def_self("new", regox_regex_new);
        klass.def("match?", regox_regex_is_match);
        klass.def("scan", regox_regex_scan);
    });
}
