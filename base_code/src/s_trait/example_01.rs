/// create trait

pub trait Summary {
    fn summarize_author(&self) -> String;

    fn summarize(&self) -> String {
        format!("(更多阅读 {}... )", self.summarize_author())
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize_author(&self) -> String {
        format!("@{}", self.author)
    }
    fn summarize(&self) -> String {
        format!("{}, by {} {}", self.headline, self.author, self.location)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
}

pub fn notify<T: Summary>(item: T) {
    println!("爆炸新闻! {}", item.summarize())
}

#[test]
fn example_1() {
    let tweet = Tweet {
        username: "马书".to_string(),
        content: "这里是马书内容".to_string(),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet:{}", tweet.summarize());

    let article = NewsArticle {
        headline: "这里是标题".to_string(),
        location: "这里是位置".to_string(),
        author: "这里是作者".to_string(),
        content: "这里是内容".to_string(),
    };
    println!("新书推荐！ {}", article.summarize());

    notify(article)
}
