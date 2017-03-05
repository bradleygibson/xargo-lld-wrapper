#![cfg_attr(feature="clippy", feature(plugin))]
#![cfg_attr(feature="clippy", plugin(clippy))]
#![deny(missing_debug_implementations, missing_copy_implementations, trivial_casts, trivial_numeric_casts,
unsafe_code, unused_import_braces, unused_qualifications)]
#![allow(unused_variables)]
#![feature(inclusive_range_syntax)]
#![feature(associated_consts)]

trait ArgsExtensionMethods<'a> where Self: IntoIterator<Item = &'a str> {
    const NOSTARTFILES_SWITCH: &'static str = "-nostartfiles";
    const WL_SWITCH_PREFIX: &'static str = "-Wl,";
    const OUTPUT_SWITCH: &'static str = "-o";

    fn get_output_filename(self) -> Option<&'a str>;
    fn remove_output_filename_switches_and_params(self) -> Vec<&'a str>;
    fn remove_nostartfiles_switches(self) -> Vec<&'a str>;
    fn remove_wl_switches(self) -> Vec<&'a str>;
}

//TODO: Return (last occurrence of) OUTPUT_FILENAME parameter, if present
impl<'a, T> ArgsExtensionMethods<'a> for T where T: IntoIterator<Item = &'a str> {
    fn get_output_filename(self) -> Option<&'a str> {
        self.into_iter()
            .skip_while(|&el| el != Self::OUTPUT_SWITCH)
            .skip(1)    //move past -o switch
            .next()
    }

    //Delete all of both the OUTPUT_SWITCH and the immediately following argument (OUTPUT_FILENAME) if one is present
    fn remove_output_filename_switches_and_params(self) -> Vec<&'a str> {
        self.into_iter()
            .scan(false, |found_switch, el| match *found_switch {
                true => { *found_switch = false; Some(None) },
                false => match el == Self::OUTPUT_SWITCH {
                    true => { *found_switch = true; Some(None) },
                    false => Some(Some(el)) }})
            .filter(|el| el.is_some())
            .map(|el| el.unwrap())
            .collect::<Vec<_>>()
    }

    fn remove_nostartfiles_switches(self) -> Vec<&'a str> {
        self.into_iter()
            .filter(|&el| el != Self::NOSTARTFILES_SWITCH)
            .collect::<Vec<_>>()
    }

    fn remove_wl_switches(self) -> Vec<&'a str> {
        self.into_iter()
            .map(|el| match el.starts_with(Self::WL_SWITCH_PREFIX) {
                true => &el[Self::WL_SWITCH_PREFIX.as_bytes().len()..],
                false => &el, })
            .filter(|el| !el.trim().is_empty())
            .collect::<Vec<_>>()
    }
}

//Auto-convert from Vec<String> to Vec<&str>; https://is.gd/UbOlU2  (implement std::convert::From<Vec<String>> for Vec<&str>)
pub fn lib_main<'a, T>(args: T) where T: IntoIterator<Item=&'a str>
{
}

#[cfg(test)]
mod tests;


