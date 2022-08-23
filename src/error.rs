use tea_codec::define_scope;

define_scope! {
	WapcGuest {
		Host as v => Host, v.0.as_str();
		BadDispatch as v => BadDispatch, v.0.as_str();
	}
}

#[derive(Debug)]
pub struct Host(pub String);

#[derive(Debug)]
pub struct BadDispatch(pub String);
