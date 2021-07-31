#[cfg(test)]
mod tests {
    #[test]
    fn test_dotenv() {
        dotenv::dotenv().ok(); // ここで .evn ファイルの値を読み込んで、環境変数として参照できるようにしている。
        assert_eq!(std::env::var("URL").unwrap(), "localhost"); // 環境変数経由で参照
    }

    #[test]
    fn test_dotenv_var() {
        assert_eq!(dotenv::var("URL").unwrap(), "localhost"); // 直接キーを指定して参照することもできる
    }
}
