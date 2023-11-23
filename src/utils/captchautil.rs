use captcha::Captcha;
use captcha::filters::Noise;

pub fn get_captcha()->Option<String>{
    Captcha::new()
    .add_chars(4)
    .apply_filter(Noise::new(0.1))
    .view(220, 120)
    .as_base64()
}

#[cfg(test)]
mod test{
    use super::get_captcha;

    #[test]
    pub fn test_captcha(){
        get_captcha();
    }

}