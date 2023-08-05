#[cfg(test)]
mod tests {
    #[test]
    fn dotenv() {
        // .env ファイルの値を読み込んで、環境変数として参照できるようにしている。
        dotenvy::dotenv().unwrap();
        // 環境変数経由で参照
        assert_eq!(std::env::var("URL").unwrap(), "localhost");
    }

    #[test]
    fn dotenv_var() {
        // 直接キーを指定して参照することもできる
        assert_eq!(dotenvy::var("URL").unwrap(), "localhost");
    }
}
