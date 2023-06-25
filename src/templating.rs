use sailfish::TemplateOnce;

pub struct BlogArticleTemplate {
    pub title:    String,
    pub pub_date: String,
}

#[derive(TemplateOnce)]
#[template(path = "homepage.stpl")]

pub struct BlogHomePageTemplate {
    pub articles: Vec<BlogArticleTemplate>,
}
