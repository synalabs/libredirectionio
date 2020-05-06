extern crate redirectionio;

#[rustfmt::skip]
mod generated_tests {

use redirectionio::router::Router;
use redirectionio::api::Rule;
use redirectionio::http::Request;
use redirectionio::action::Action;


fn setup_00_common_rules() -> Router<Rule> {
    let mut router = Router::<Rule>::default();

    let route_1: Rule = serde_json::from_str(r#"{"body_filters":null,"id":"simple-foobar-rule","markers":null,"rank":0,"redirect_code":302,"source":{"host":"","path":"/foo","query":""},"target":"/bar"}"#).expect("cannot deserialize");
    router.insert(route_1.into_route());

    router
}


#[test]
fn test_00_common_rules_1() {
    let router = setup_00_common_rules();
    let request = Request::new(r#"/foo"#.to_string(),None,None,
    None).to_http_request().expect("");
    let matched = router.match_request(&request);

    assert_eq!(!matched.is_empty(), true);

    let action = Action::from_routes_rule(matched, &request);

    assert_eq!(action.get_status_code(0), 302);
    let headers = action.filter_headers(Vec::new(), 0);
    assert_eq!(headers.len(), 1);

    let target_header = headers.first().unwrap();
    assert_eq!(target_header.name, "Location");
    assert_eq!(target_header.value, r#"/bar"#);
}

#[test]
fn test_00_common_rules_2() {
    let router = setup_00_common_rules();
    let request = Request::new(r#"/foo2"#.to_string(),None,None,
    None).to_http_request().expect("");
    let matched = router.match_request(&request);

    assert_eq!(!matched.is_empty(), false);

}


fn setup_01_straight_rule_match() -> Router<Rule> {
    let mut router = Router::<Rule>::default();

    let route_1: Rule = serde_json::from_str(r#"{"body_filters":null,"id":"any-host-path","markers":null,"rank":0,"redirect_code":301,"source":{"host":"","path":"/foo","query":""},"target":"/any-host--path-only"}"#).expect("cannot deserialize");
    router.insert(route_1.into_route());

    let route_2: Rule = serde_json::from_str(r#"{"body_filters":null,"id":"any-host-path-query","markers":null,"rank":0,"redirect_code":301,"source":{"host":"","path":"/foo","query":"bar=baz"},"target":"/any-host--path-query"}"#).expect("cannot deserialize");
    router.insert(route_2.into_route());

    let route_3: Rule = serde_json::from_str(r#"{"body_filters":null,"id":"any-host-query-only","markers":null,"rank":0,"redirect_code":301,"source":{"host":"","path":"/","query":"bar=baz"},"target":"/any-host--query-only"}"#).expect("cannot deserialize");
    router.insert(route_3.into_route());

    let route_4: Rule = serde_json::from_str(r#"{"body_filters":null,"id":"empty","markers":null,"rank":0,"redirect_code":301,"source":{"host":"","path":"/","query":""},"target":"/empty"}"#).expect("cannot deserialize");
    router.insert(route_4.into_route());

    let route_5: Rule = serde_json::from_str(r#"{"body_filters":null,"id":"example-net-host-only","markers":null,"rank":0,"redirect_code":301,"source":{"host":"example.net","path":"/","query":""},"target":"/example.net--host-only"}"#).expect("cannot deserialize");
    router.insert(route_5.into_route());

    let route_6: Rule = serde_json::from_str(r#"{"body_filters":null,"id":"example-net-host-path","markers":null,"rank":0,"redirect_code":301,"source":{"host":"example.net","path":"/foo","query":""},"target":"/example.net--host-path-only"}"#).expect("cannot deserialize");
    router.insert(route_6.into_route());

    let route_7: Rule = serde_json::from_str(r#"{"body_filters":null,"id":"example-net-host-path-query","markers":null,"rank":0,"redirect_code":301,"source":{"host":"example.net","path":"/foo","query":"bar=baz"},"target":"/example.net--host-path-query"}"#).expect("cannot deserialize");
    router.insert(route_7.into_route());

    let route_8: Rule = serde_json::from_str(r#"{"body_filters":null,"id":"host","markers":null,"rank":0,"redirect_code":301,"source":{"host":"example.org","path":"/","query":""},"target":"/example.org--host-only"}"#).expect("cannot deserialize");
    router.insert(route_8.into_route());

    let route_9: Rule = serde_json::from_str(r#"{"body_filters":null,"id":"host-path-query","markers":null,"rank":0,"redirect_code":301,"source":{"host":"example.org","path":"/foo","query":"bar=baz"},"target":"/example.org--host-path-query"}"#).expect("cannot deserialize");
    router.insert(route_9.into_route());

    let route_10: Rule = serde_json::from_str(r#"{"body_filters":null,"id":"host-with-path","markers":null,"rank":0,"redirect_code":301,"source":{"host":"example.org","path":"/foo","query":""},"target":"/example.org--host-path-only"}"#).expect("cannot deserialize");
    router.insert(route_10.into_route());

    let route_11: Rule = serde_json::from_str(r#"{"body_filters":null,"id":"host-with-query","markers":null,"rank":0,"redirect_code":301,"source":{"host":"example.org","path":"/","query":"bar=baz"},"target":"/example.org--host-query-only"}"#).expect("cannot deserialize");
    router.insert(route_11.into_route());

    let route_12: Rule = serde_json::from_str(r#"{"body_filters":null,"id":"path-with-plus-sign","markers":null,"rank":0,"redirect_code":301,"source":{"host":"www.domain.nl","path":"/zwart+janstraat","query":""},"target":"/plus-sign"}"#).expect("cannot deserialize");
    router.insert(route_12.into_route());

    let route_13: Rule = serde_json::from_str(r#"{"body_filters":null,"id":"path-with-space-percent-encoded","markers":null,"rank":0,"redirect_code":301,"source":{"host":"example.net","path":"/i%20have%20space","query":""},"target":"/space"}"#).expect("cannot deserialize");
    router.insert(route_13.into_route());

    router
}


#[test]
fn test_01_straight_rule_match_1() {
    let router = setup_01_straight_rule_match();
    let request = Request::new(r#"/foo"#.to_string(),Some(r#"example.org"#.to_string()),Some(r#"http"#.to_string()),
    None).to_http_request().expect("");
    let matched = router.match_request(&request);

    assert_eq!(!matched.is_empty(), true);

    let action = Action::from_routes_rule(matched, &request);

    assert_eq!(action.get_status_code(0), 301);
    let headers = action.filter_headers(Vec::new(), 0);
    assert_eq!(headers.len(), 1);

    let target_header = headers.first().unwrap();
    assert_eq!(target_header.name, "Location");
    assert_eq!(target_header.value, r#"/example.org--host-path-only"#);
}

#[test]
fn test_01_straight_rule_match_2() {
    let router = setup_01_straight_rule_match();
    let request = Request::new(r#"/foo?bar=baz"#.to_string(),Some(r#"example.org"#.to_string()),Some(r#"http"#.to_string()),
    None).to_http_request().expect("");
    let matched = router.match_request(&request);

    assert_eq!(!matched.is_empty(), true);

    let action = Action::from_routes_rule(matched, &request);

    assert_eq!(action.get_status_code(0), 301);
    let headers = action.filter_headers(Vec::new(), 0);
    assert_eq!(headers.len(), 1);

    let target_header = headers.first().unwrap();
    assert_eq!(target_header.name, "Location");
    assert_eq!(target_header.value, r#"/example.org--host-path-query"#);
}

#[test]
fn test_01_straight_rule_match_3() {
    let router = setup_01_straight_rule_match();
    let request = Request::new(r#"/?q"#.to_string(),None,None,
    None).to_http_request().expect("");
    let matched = router.match_request(&request);

    assert_eq!(!matched.is_empty(), false);

}

#[test]
fn test_01_straight_rule_match_4() {
    let router = setup_01_straight_rule_match();
    let request = Request::new(r#"/?"#.to_string(),None,None,
    None).to_http_request().expect("");
    let matched = router.match_request(&request);

    assert_eq!(!matched.is_empty(), true);

    let action = Action::from_routes_rule(matched, &request);

    assert_eq!(action.get_status_code(0), 301);
    let headers = action.filter_headers(Vec::new(), 0);
    assert_eq!(headers.len(), 1);

    let target_header = headers.first().unwrap();
    assert_eq!(target_header.name, "Location");
    assert_eq!(target_header.value, r#"/empty"#);
}

#[test]
fn test_01_straight_rule_match_5() {
    let router = setup_01_straight_rule_match();
    let request = Request::new(r#"?bar2=baz"#.to_string(),Some(r#"example.org"#.to_string()),Some(r#"http"#.to_string()),
    None).to_http_request().expect("");
    let matched = router.match_request(&request);

    assert_eq!(!matched.is_empty(), false);

}

#[test]
fn test_01_straight_rule_match_6() {
    let router = setup_01_straight_rule_match();
    let request = Request::new(r#"/foo?bar=baz"#.to_string(),Some(r#"foobar.org"#.to_string()),Some(r#"http"#.to_string()),
    None).to_http_request().expect("");
    let matched = router.match_request(&request);

    assert_eq!(!matched.is_empty(), true);

    let action = Action::from_routes_rule(matched, &request);

    assert_eq!(action.get_status_code(0), 301);
    let headers = action.filter_headers(Vec::new(), 0);
    assert_eq!(headers.len(), 1);

    let target_header = headers.first().unwrap();
    assert_eq!(target_header.name, "Location");
    assert_eq!(target_header.value, r#"/any-host--path-query"#);
}

#[test]
fn test_01_straight_rule_match_7() {
    let router = setup_01_straight_rule_match();
    let request = Request::new(r#"/foo"#.to_string(),Some(r#"example.net"#.to_string()),Some(r#"http"#.to_string()),
    None).to_http_request().expect("");
    let matched = router.match_request(&request);

    assert_eq!(!matched.is_empty(), true);

    let action = Action::from_routes_rule(matched, &request);

    assert_eq!(action.get_status_code(0), 301);
    let headers = action.filter_headers(Vec::new(), 0);
    assert_eq!(headers.len(), 1);

    let target_header = headers.first().unwrap();
    assert_eq!(target_header.name, "Location");
    assert_eq!(target_header.value, r#"/example.net--host-path-only"#);
}

#[test]
fn test_01_straight_rule_match_8() {
    let router = setup_01_straight_rule_match();
    let request = Request::new(r#"/foo?bar=baz"#.to_string(),Some(r#"example.net"#.to_string()),Some(r#"http"#.to_string()),
    None).to_http_request().expect("");
    let matched = router.match_request(&request);

    assert_eq!(!matched.is_empty(), true);

    let action = Action::from_routes_rule(matched, &request);

    assert_eq!(action.get_status_code(0), 301);
    let headers = action.filter_headers(Vec::new(), 0);
    assert_eq!(headers.len(), 1);

    let target_header = headers.first().unwrap();
    assert_eq!(target_header.name, "Location");
    assert_eq!(target_header.value, r#"/example.net--host-path-query"#);
}

#[test]
fn test_01_straight_rule_match_9() {
    let router = setup_01_straight_rule_match();
    let request = Request::new(r#"/i%20have%20space"#.to_string(),Some(r#"example.net"#.to_string()),Some(r#"http"#.to_string()),
    None).to_http_request().expect("");
    let matched = router.match_request(&request);

    assert_eq!(!matched.is_empty(), true);

    let action = Action::from_routes_rule(matched, &request);

    assert_eq!(action.get_status_code(0), 301);
    let headers = action.filter_headers(Vec::new(), 0);
    assert_eq!(headers.len(), 1);

    let target_header = headers.first().unwrap();
    assert_eq!(target_header.name, "Location");
    assert_eq!(target_header.value, r#"/space"#);
}

#[test]
fn test_01_straight_rule_match_10() {
    let router = setup_01_straight_rule_match();
    let request = Request::new(r#"/i have space"#.to_string(),Some(r#"example.net"#.to_string()),Some(r#"http"#.to_string()),
    None).to_http_request().expect("");
    let matched = router.match_request(&request);

    assert_eq!(!matched.is_empty(), true);

    let action = Action::from_routes_rule(matched, &request);

    assert_eq!(action.get_status_code(0), 301);
    let headers = action.filter_headers(Vec::new(), 0);
    assert_eq!(headers.len(), 1);

    let target_header = headers.first().unwrap();
    assert_eq!(target_header.name, "Location");
    assert_eq!(target_header.value, r#"/space"#);
}

#[test]
fn test_01_straight_rule_match_11() {
    let router = setup_01_straight_rule_match();
    let request = Request::new(r#"/zwart+janstraat"#.to_string(),Some(r#"www.domain.nl"#.to_string()),Some(r#"http"#.to_string()),
    None).to_http_request().expect("");
    let matched = router.match_request(&request);

    assert_eq!(!matched.is_empty(), true);

    let action = Action::from_routes_rule(matched, &request);

    assert_eq!(action.get_status_code(0), 301);
    let headers = action.filter_headers(Vec::new(), 0);
    assert_eq!(headers.len(), 1);

    let target_header = headers.first().unwrap();
    assert_eq!(target_header.name, "Location");
    assert_eq!(target_header.value, r#"/plus-sign"#);
}


fn setup_03_priority_match() -> Router<Rule> {
    let mut router = Router::<Rule>::default();

    let route_1: Rule = serde_json::from_str(r#"{"body_filters":null,"id":"complex-example","markers":null,"rank":0,"redirect_code":301,"source":{"host":"","path":"/foo","query":""},"target":"/complex-example-org"}"#).expect("cannot deserialize");
    router.insert(route_1.into_route());

    let route_2: Rule = serde_json::from_str(r#"{"body_filters":null,"id":"complex-example-net","markers":null,"rank":0,"redirect_code":301,"source":{"host":"","path":"/foo","query":""},"target":"/complex-example-net"}"#).expect("cannot deserialize");
    router.insert(route_2.into_route());

    let route_3: Rule = serde_json::from_str(r#"{"body_filters":null,"id":"straight-any-host","markers":null,"rank":0,"redirect_code":301,"source":{"host":"","path":"/foo","query":""},"target":"/straight-any-host"}"#).expect("cannot deserialize");
    router.insert(route_3.into_route());

    let route_4: Rule = serde_json::from_str(r#"{"body_filters":null,"id":"straight-example-net","markers":null,"rank":0,"redirect_code":301,"source":{"host":"example.net","path":"/foo","query":""},"target":"/straight-example-net"}"#).expect("cannot deserialize");
    router.insert(route_4.into_route());

    let route_5: Rule = serde_json::from_str(r#"{"body_filters":null,"id":"straigth-example","markers":null,"rank":0,"redirect_code":301,"source":{"host":"example.org","path":"/foo","query":""},"target":"/straight-example-org"}"#).expect("cannot deserialize");
    router.insert(route_5.into_route());

    let route_6: Rule = serde_json::from_str(r#"{"body_filters":null,"id":"straigth-example-same-rank-but-after","markers":null,"rank":0,"redirect_code":301,"source":{"host":"example.fr","path":"/foo","query":""},"target":"/straight-example-fr"}"#).expect("cannot deserialize");
    router.insert(route_6.into_route());

    router
}


#[test]
fn test_03_priority_match_1() {
    let router = setup_03_priority_match();
    let request = Request::new(r#"/foo"#.to_string(),Some(r#"example.org"#.to_string()),Some(r#"http"#.to_string()),
    None).to_http_request().expect("");
    let matched = router.match_request(&request);

    assert_eq!(!matched.is_empty(), true);

    let action = Action::from_routes_rule(matched, &request);

    assert_eq!(action.get_status_code(0), 301);
    let headers = action.filter_headers(Vec::new(), 0);
    assert_eq!(headers.len(), 1);

    let target_header = headers.first().unwrap();
    assert_eq!(target_header.name, "Location");
    assert_eq!(target_header.value, r#"/straight-example-org"#);
}

#[test]
fn test_03_priority_match_2() {
    let router = setup_03_priority_match();
    let request = Request::new(r#"/foo"#.to_string(),Some(r#"example.com"#.to_string()),Some(r#"http"#.to_string()),
    None).to_http_request().expect("");
    let matched = router.match_request(&request);

    assert_eq!(!matched.is_empty(), true);

    let action = Action::from_routes_rule(matched, &request);

    assert_eq!(action.get_status_code(0), 301);
    let headers = action.filter_headers(Vec::new(), 0);
    assert_eq!(headers.len(), 1);

    let target_header = headers.first().unwrap();
    assert_eq!(target_header.name, "Location");
    assert_eq!(target_header.value, r#"/straight-any-host"#);
}

#[test]
fn test_03_priority_match_3() {
    let router = setup_03_priority_match();
    let request = Request::new(r#"/foo"#.to_string(),Some(r#"example.net"#.to_string()),Some(r#"http"#.to_string()),
    None).to_http_request().expect("");
    let matched = router.match_request(&request);

    assert_eq!(!matched.is_empty(), true);

    let action = Action::from_routes_rule(matched, &request);

    assert_eq!(action.get_status_code(0), 301);
    let headers = action.filter_headers(Vec::new(), 0);
    assert_eq!(headers.len(), 1);

    let target_header = headers.first().unwrap();
    assert_eq!(target_header.name, "Location");
    assert_eq!(target_header.value, r#"/straight-example-net"#);
}

#[test]
fn test_03_priority_match_4() {
    let router = setup_03_priority_match();
    let request = Request::new(r#"/foo"#.to_string(),Some(r#"example.fr"#.to_string()),Some(r#"http"#.to_string()),
    None).to_http_request().expect("");
    let matched = router.match_request(&request);

    assert_eq!(!matched.is_empty(), true);

    let action = Action::from_routes_rule(matched, &request);

    assert_eq!(action.get_status_code(0), 301);
    let headers = action.filter_headers(Vec::new(), 0);
    assert_eq!(headers.len(), 1);

    let target_header = headers.first().unwrap();
    assert_eq!(target_header.name, "Location");
    assert_eq!(target_header.value, r#"/straight-example-fr"#);
}


fn setup_04_rfc3986_relative_references() -> Router<Rule> {
    let mut router = Router::<Rule>::default();

    let route_1: Rule = serde_json::from_str(r#"{"body_filters":null,"id":"doublepathSource","markers":null,"rank":0,"redirect_code":301,"source":{"host":"","path":"//xyz","query":""},"target":"/xyz"}"#).expect("cannot deserialize");
    router.insert(route_1.into_route());

    let route_2: Rule = serde_json::from_str(r#"{"body_filters":null,"id":"doublepathSourceWithHost","markers":null,"rank":0,"redirect_code":301,"source":{"host":"yolo.com","path":"//doubledragon","query":""},"target":"/simpledragon"}"#).expect("cannot deserialize");
    router.insert(route_2.into_route());

    let route_3: Rule = serde_json::from_str(r#"{"body_filters":null,"id":"doublepathTarget","markers":null,"rank":0,"redirect_code":301,"source":{"host":"","path":"/source","query":""},"target":"//target"}"#).expect("cannot deserialize");
    router.insert(route_3.into_route());

    router
}


#[test]
fn test_04_rfc3986_relative_references_1() {
    let router = setup_04_rfc3986_relative_references();
    let request = Request::new(r#"//xyz"#.to_string(),Some(r#"example.org"#.to_string()),Some(r#"http"#.to_string()),
    None).to_http_request().expect("");
    let matched = router.match_request(&request);

    assert_eq!(!matched.is_empty(), true);

    let action = Action::from_routes_rule(matched, &request);

    assert_eq!(action.get_status_code(0), 301);
    let headers = action.filter_headers(Vec::new(), 0);
    assert_eq!(headers.len(), 1);

    let target_header = headers.first().unwrap();
    assert_eq!(target_header.name, "Location");
    assert_eq!(target_header.value, r#"/xyz"#);
}

#[test]
fn test_04_rfc3986_relative_references_2() {
    let router = setup_04_rfc3986_relative_references();
    let request = Request::new(r#"/xyz"#.to_string(),Some(r#"example.org"#.to_string()),Some(r#"http"#.to_string()),
    None).to_http_request().expect("");
    let matched = router.match_request(&request);

    assert_eq!(!matched.is_empty(), false);

}

#[test]
fn test_04_rfc3986_relative_references_3() {
    let router = setup_04_rfc3986_relative_references();
    let request = Request::new(r#"/source"#.to_string(),Some(r#"example.org"#.to_string()),Some(r#"http"#.to_string()),
    None).to_http_request().expect("");
    let matched = router.match_request(&request);

    assert_eq!(!matched.is_empty(), true);

    let action = Action::from_routes_rule(matched, &request);

    assert_eq!(action.get_status_code(0), 301);
    let headers = action.filter_headers(Vec::new(), 0);
    assert_eq!(headers.len(), 1);

    let target_header = headers.first().unwrap();
    assert_eq!(target_header.name, "Location");
    assert_eq!(target_header.value, r#"//target"#);
}

#[test]
fn test_04_rfc3986_relative_references_4() {
    let router = setup_04_rfc3986_relative_references();
    let request = Request::new(r#"//doubledragon"#.to_string(),Some(r#"yolo.com"#.to_string()),Some(r#"http"#.to_string()),
    None).to_http_request().expect("");
    let matched = router.match_request(&request);

    assert_eq!(!matched.is_empty(), true);

    let action = Action::from_routes_rule(matched, &request);

    assert_eq!(action.get_status_code(0), 301);
    let headers = action.filter_headers(Vec::new(), 0);
    assert_eq!(headers.len(), 1);

    let target_header = headers.first().unwrap();
    assert_eq!(target_header.name, "Location");
    assert_eq!(target_header.value, r#"/simpledragon"#);
}


fn setup_05_query_parameters_order() -> Router<Rule> {
    let mut router = Router::<Rule>::default();

    let route_1: Rule = serde_json::from_str(r#"{"body_filters":null,"id":"rule-inverted-with-query-parameters","markers":null,"rank":0,"redirect_code":302,"source":{"host":"","path":"/foo","query":"c=c&b=b"},"target":"/bar-inverted"}"#).expect("cannot deserialize");
    router.insert(route_1.into_route());

    let route_2: Rule = serde_json::from_str(r#"{"body_filters":null,"id":"rule-with-query-parameters","markers":null,"rank":0,"redirect_code":302,"source":{"host":"","path":"/foo","query":"a=a&b=b"},"target":"/bar"}"#).expect("cannot deserialize");
    router.insert(route_2.into_route());

    router
}


#[test]
fn test_05_query_parameters_order_1() {
    let router = setup_05_query_parameters_order();
    let request = Request::new(r#"/foo?a=a&b=b"#.to_string(),None,None,
    None).to_http_request().expect("");
    let matched = router.match_request(&request);

    assert_eq!(!matched.is_empty(), true);

    let action = Action::from_routes_rule(matched, &request);

    assert_eq!(action.get_status_code(0), 302);
    let headers = action.filter_headers(Vec::new(), 0);
    assert_eq!(headers.len(), 1);

    let target_header = headers.first().unwrap();
    assert_eq!(target_header.name, "Location");
    assert_eq!(target_header.value, r#"/bar"#);
}

#[test]
fn test_05_query_parameters_order_2() {
    let router = setup_05_query_parameters_order();
    let request = Request::new(r#"/foo?b=b&a=a"#.to_string(),None,None,
    None).to_http_request().expect("");
    let matched = router.match_request(&request);

    assert_eq!(!matched.is_empty(), true);

    let action = Action::from_routes_rule(matched, &request);

    assert_eq!(action.get_status_code(0), 302);
    let headers = action.filter_headers(Vec::new(), 0);
    assert_eq!(headers.len(), 1);

    let target_header = headers.first().unwrap();
    assert_eq!(target_header.name, "Location");
    assert_eq!(target_header.value, r#"/bar"#);
}

#[test]
fn test_05_query_parameters_order_3() {
    let router = setup_05_query_parameters_order();
    let request = Request::new(r#"/foo?a=a&b=b&c=c"#.to_string(),None,None,
    None).to_http_request().expect("");
    let matched = router.match_request(&request);

    assert_eq!(!matched.is_empty(), false);

}

#[test]
fn test_05_query_parameters_order_4() {
    let router = setup_05_query_parameters_order();
    let request = Request::new(r#"/foo?b=b&c=c"#.to_string(),None,None,
    None).to_http_request().expect("");
    let matched = router.match_request(&request);

    assert_eq!(!matched.is_empty(), true);

    let action = Action::from_routes_rule(matched, &request);

    assert_eq!(action.get_status_code(0), 302);
    let headers = action.filter_headers(Vec::new(), 0);
    assert_eq!(headers.len(), 1);

    let target_header = headers.first().unwrap();
    assert_eq!(target_header.name, "Location");
    assert_eq!(target_header.value, r#"/bar-inverted"#);
}

#[test]
fn test_05_query_parameters_order_5() {
    let router = setup_05_query_parameters_order();
    let request = Request::new(r#"/foo?c=c&b=b"#.to_string(),None,None,
    None).to_http_request().expect("");
    let matched = router.match_request(&request);

    assert_eq!(!matched.is_empty(), true);

    let action = Action::from_routes_rule(matched, &request);

    assert_eq!(action.get_status_code(0), 302);
    let headers = action.filter_headers(Vec::new(), 0);
    assert_eq!(headers.len(), 1);

    let target_header = headers.first().unwrap();
    assert_eq!(target_header.name, "Location");
    assert_eq!(target_header.value, r#"/bar-inverted"#);
}


fn setup_06_emojis() -> Router<Rule> {
    let mut router = Router::<Rule>::default();

    let route_1: Rule = serde_json::from_str(r#"{"body_filters":null,"id":"simple-emoji-rule","markers":null,"rank":0,"redirect_code":302,"source":{"host":"","path":"/🍕","query":""},"target":"/bar"}"#).expect("cannot deserialize");
    router.insert(route_1.into_route());

    router
}


#[test]
fn test_06_emojis_1() {
    let router = setup_06_emojis();
    let request = Request::new(r#"/🍕"#.to_string(),Some(r#"example.org"#.to_string()),Some(r#"http"#.to_string()),
    None).to_http_request().expect("");
    let matched = router.match_request(&request);

    assert_eq!(!matched.is_empty(), true);

    let action = Action::from_routes_rule(matched, &request);

    assert_eq!(action.get_status_code(0), 302);
    let headers = action.filter_headers(Vec::new(), 0);
    assert_eq!(headers.len(), 1);

    let target_header = headers.first().unwrap();
    assert_eq!(target_header.name, "Location");
    assert_eq!(target_header.value, r#"/bar"#);
}


fn setup_action_seo_override_meta_author() -> Router<Rule> {
    let mut router = Router::<Rule>::default();

    let route_1: Rule = serde_json::from_str(r#"{"body_filters":[{"action":"append_child","css_selector":"meta[name=\"author\"]","element_tree":["html","head"],"value":"<meta name=\"author\" content=\"Author name\" />"},{"action":"replace","css_selector":"meta[name=\"author\"]","element_tree":["html","head","meta"],"value":"<meta name=\"author\" content=\"Author name\" />"}],"id":"override-meta-author-rule","markers":null,"rank":0,"redirect_code":null,"source":{"host":"","path":"/source","query":""},"target":null}"#).expect("cannot deserialize");
    router.insert(route_1.into_route());

    router
}


#[test]
fn test_action_seo_override_meta_author_1() {
    let router = setup_action_seo_override_meta_author();
    let request = Request::new(r#"/source"#.to_string(),None,None,
    None).to_http_request().expect("");
    let matched = router.match_request(&request);

    assert_eq!(!matched.is_empty(), true);

    let action = Action::from_routes_rule(matched, &request);

    let body_filter = action.create_filter_body(0);
    assert_eq!(body_filter.is_some(), true);

    let mut filter = body_filter.unwrap();
    let mut new_body = filter.filter(r#"<html><head><meta /></head></html>"#.to_string());
    new_body.push_str(filter.end().as_str());
    assert_eq!(new_body, r#"<html><head><meta /><meta name="author" content="Author name" /></head></html>"#)
}

#[test]
fn test_action_seo_override_meta_author_2() {
    let router = setup_action_seo_override_meta_author();
    let request = Request::new(r#"/source"#.to_string(),None,None,
    None).to_http_request().expect("");
    let matched = router.match_request(&request);

    assert_eq!(!matched.is_empty(), true);

    let action = Action::from_routes_rule(matched, &request);

    let body_filter = action.create_filter_body(0);
    assert_eq!(body_filter.is_some(), true);

    let mut filter = body_filter.unwrap();
    let mut new_body = filter.filter(r#"<html><head><meta name="author" /></head></html>"#.to_string());
    new_body.push_str(filter.end().as_str());
    assert_eq!(new_body, r#"<html><head><meta name="author" content="Author name" /></head></html>"#)
}

#[test]
fn test_action_seo_override_meta_author_3() {
    let router = setup_action_seo_override_meta_author();
    let request = Request::new(r#"/source"#.to_string(),None,None,
    None).to_http_request().expect("");
    let matched = router.match_request(&request);

    assert_eq!(!matched.is_empty(), true);

    let action = Action::from_routes_rule(matched, &request);

    let body_filter = action.create_filter_body(0);
    assert_eq!(body_filter.is_some(), true);

    let mut filter = body_filter.unwrap();
    let mut new_body = filter.filter(r#"<html><head><meta name="author" content="Old Author name" /></head></html>"#.to_string());
    new_body.push_str(filter.end().as_str());
    assert_eq!(new_body, r#"<html><head><meta name="author" content="Author name" /></head></html>"#)
}

#[test]
fn test_action_seo_override_meta_author_4() {
    let router = setup_action_seo_override_meta_author();
    let request = Request::new(r#"/source"#.to_string(),None,None,
    None).to_http_request().expect("");
    let matched = router.match_request(&request);

    assert_eq!(!matched.is_empty(), true);

    let action = Action::from_routes_rule(matched, &request);

    let body_filter = action.create_filter_body(0);
    assert_eq!(body_filter.is_some(), true);

    let mut filter = body_filter.unwrap();
    let mut new_body = filter.filter(r#"<html><head><meta name="author" /><meta name="author" content="Old Author name" /></head></html>"#.to_string());
    new_body.push_str(filter.end().as_str());
    assert_eq!(new_body, r#"<html><head><meta name="author" content="Author name" /><meta name="author" content="Author name" /></head></html>"#)
}

#[test]
fn test_action_seo_override_meta_author_5() {
    let router = setup_action_seo_override_meta_author();
    let request = Request::new(r#"/source"#.to_string(),None,None,
    None).to_http_request().expect("");
    let matched = router.match_request(&request);

    assert_eq!(!matched.is_empty(), true);

    let action = Action::from_routes_rule(matched, &request);

    let body_filter = action.create_filter_body(0);
    assert_eq!(body_filter.is_some(), true);

    let mut filter = body_filter.unwrap();
    let mut new_body = filter.filter(r#"<html><head><meta name="author" content="Old first Author name" /><meta name="author" content="Old second Author name" /></head></html>"#.to_string());
    new_body.push_str(filter.end().as_str());
    assert_eq!(new_body, r#"<html><head><meta name="author" content="Author name" /><meta name="author" content="Author name" /></head></html>"#)
}

#[test]
fn test_action_seo_override_meta_author_6() {
    let router = setup_action_seo_override_meta_author();
    let request = Request::new(r#"/source"#.to_string(),None,None,
    None).to_http_request().expect("");
    let matched = router.match_request(&request);

    assert_eq!(!matched.is_empty(), true);

    let action = Action::from_routes_rule(matched, &request);

    let body_filter = action.create_filter_body(0);
    assert_eq!(body_filter.is_some(), true);

    let mut filter = body_filter.unwrap();
    let mut new_body = filter.filter(r#"<html><head><meta></head></html>"#.to_string());
    new_body.push_str(filter.end().as_str());
    assert_eq!(new_body, r#"<html><head><meta><meta name="author" content="Author name" /></head></html>"#)
}

#[test]
fn test_action_seo_override_meta_author_7() {
    let router = setup_action_seo_override_meta_author();
    let request = Request::new(r#"/source"#.to_string(),None,None,
    None).to_http_request().expect("");
    let matched = router.match_request(&request);

    assert_eq!(!matched.is_empty(), true);

    let action = Action::from_routes_rule(matched, &request);

    let body_filter = action.create_filter_body(0);
    assert_eq!(body_filter.is_some(), true);

    let mut filter = body_filter.unwrap();
    let mut new_body = filter.filter(r#"<html><head><meta name="author"></head></html>"#.to_string());
    new_body.push_str(filter.end().as_str());
    assert_eq!(new_body, r#"<html><head><meta name="author" content="Author name" /></head></html>"#)
}

#[test]
fn test_action_seo_override_meta_author_8() {
    let router = setup_action_seo_override_meta_author();
    let request = Request::new(r#"/source"#.to_string(),None,None,
    None).to_http_request().expect("");
    let matched = router.match_request(&request);

    assert_eq!(!matched.is_empty(), true);

    let action = Action::from_routes_rule(matched, &request);

    let body_filter = action.create_filter_body(0);
    assert_eq!(body_filter.is_some(), true);

    let mut filter = body_filter.unwrap();
    let mut new_body = filter.filter(r#"<html><head><meta name="author" content="Old Author name"></head></html>"#.to_string());
    new_body.push_str(filter.end().as_str());
    assert_eq!(new_body, r#"<html><head><meta name="author" content="Author name" /></head></html>"#)
}


fn setup_action_seo_override_meta_description() -> Router<Rule> {
    let mut router = Router::<Rule>::default();

    let route_1: Rule = serde_json::from_str(r#"{"body_filters":[{"action":"append_child","css_selector":"meta[name=\"description\"]","element_tree":["html","head"],"value":"<meta name=\"description\" content=\"New Description\" />"},{"action":"replace","css_selector":"meta[name=\"description\"]","element_tree":["html","head","meta"],"value":"<meta name=\"description\" content=\"New Description\" />"}],"id":"override-meta-description-rule","markers":null,"rank":0,"redirect_code":null,"source":{"host":"","path":"/source","query":""},"target":null}"#).expect("cannot deserialize");
    router.insert(route_1.into_route());

    router
}


#[test]
fn test_action_seo_override_meta_description_1() {
    let router = setup_action_seo_override_meta_description();
    let request = Request::new(r#"/source"#.to_string(),None,None,
    None).to_http_request().expect("");
    let matched = router.match_request(&request);

    assert_eq!(!matched.is_empty(), true);

    let action = Action::from_routes_rule(matched, &request);

    let body_filter = action.create_filter_body(0);
    assert_eq!(body_filter.is_some(), true);

    let mut filter = body_filter.unwrap();
    let mut new_body = filter.filter(r#"<html><head><meta /></head></html>"#.to_string());
    new_body.push_str(filter.end().as_str());
    assert_eq!(new_body, r#"<html><head><meta /><meta name="description" content="New Description" /></head></html>"#)
}

#[test]
fn test_action_seo_override_meta_description_2() {
    let router = setup_action_seo_override_meta_description();
    let request = Request::new(r#"/source"#.to_string(),None,None,
    None).to_http_request().expect("");
    let matched = router.match_request(&request);

    assert_eq!(!matched.is_empty(), true);

    let action = Action::from_routes_rule(matched, &request);

    let body_filter = action.create_filter_body(0);
    assert_eq!(body_filter.is_some(), true);

    let mut filter = body_filter.unwrap();
    let mut new_body = filter.filter(r#"<html><head><meta name="description" /></head></html>"#.to_string());
    new_body.push_str(filter.end().as_str());
    assert_eq!(new_body, r#"<html><head><meta name="description" content="New Description" /></head></html>"#)
}

#[test]
fn test_action_seo_override_meta_description_3() {
    let router = setup_action_seo_override_meta_description();
    let request = Request::new(r#"/source"#.to_string(),None,None,
    None).to_http_request().expect("");
    let matched = router.match_request(&request);

    assert_eq!(!matched.is_empty(), true);

    let action = Action::from_routes_rule(matched, &request);

    let body_filter = action.create_filter_body(0);
    assert_eq!(body_filter.is_some(), true);

    let mut filter = body_filter.unwrap();
    let mut new_body = filter.filter(r#"<html><head><meta name="description" content="Old Description" /></head></html>"#.to_string());
    new_body.push_str(filter.end().as_str());
    assert_eq!(new_body, r#"<html><head><meta name="description" content="New Description" /></head></html>"#)
}

#[test]
fn test_action_seo_override_meta_description_4() {
    let router = setup_action_seo_override_meta_description();
    let request = Request::new(r#"/source"#.to_string(),None,None,
    None).to_http_request().expect("");
    let matched = router.match_request(&request);

    assert_eq!(!matched.is_empty(), true);

    let action = Action::from_routes_rule(matched, &request);

    let body_filter = action.create_filter_body(0);
    assert_eq!(body_filter.is_some(), true);

    let mut filter = body_filter.unwrap();
    let mut new_body = filter.filter(r#"<html><head><meta></head></html>"#.to_string());
    new_body.push_str(filter.end().as_str());
    assert_eq!(new_body, r#"<html><head><meta><meta name="description" content="New Description" /></head></html>"#)
}

#[test]
fn test_action_seo_override_meta_description_5() {
    let router = setup_action_seo_override_meta_description();
    let request = Request::new(r#"/source"#.to_string(),None,None,
    None).to_http_request().expect("");
    let matched = router.match_request(&request);

    assert_eq!(!matched.is_empty(), true);

    let action = Action::from_routes_rule(matched, &request);

    let body_filter = action.create_filter_body(0);
    assert_eq!(body_filter.is_some(), true);

    let mut filter = body_filter.unwrap();
    let mut new_body = filter.filter(r#"<html><head><meta name="description"></head></html>"#.to_string());
    new_body.push_str(filter.end().as_str());
    assert_eq!(new_body, r#"<html><head><meta name="description" content="New Description" /></head></html>"#)
}

#[test]
fn test_action_seo_override_meta_description_6() {
    let router = setup_action_seo_override_meta_description();
    let request = Request::new(r#"/source"#.to_string(),None,None,
    None).to_http_request().expect("");
    let matched = router.match_request(&request);

    assert_eq!(!matched.is_empty(), true);

    let action = Action::from_routes_rule(matched, &request);

    let body_filter = action.create_filter_body(0);
    assert_eq!(body_filter.is_some(), true);

    let mut filter = body_filter.unwrap();
    let mut new_body = filter.filter(r#"<html><head><meta name="description" content="Old Description"></head></html>"#.to_string());
    new_body.push_str(filter.end().as_str());
    assert_eq!(new_body, r#"<html><head><meta name="description" content="New Description" /></head></html>"#)
}


fn setup_action_seo_override_meta_keywords() -> Router<Rule> {
    let mut router = Router::<Rule>::default();

    let route_1: Rule = serde_json::from_str(r#"{"body_filters":[{"action":"append_child","css_selector":"meta[name=\"keywords\"]","element_tree":["html","head"],"value":"<meta name=\"keywords\" content=\"some, keywords, here\" />"},{"action":"replace","css_selector":"meta[name=\"keywords\"]","element_tree":["html","head","meta"],"value":"<meta name=\"keywords\" content=\"some, keywords, here\" />"}],"id":"override-meta-keywords-rule","markers":null,"rank":0,"redirect_code":null,"source":{"host":"","path":"/source","query":""},"target":null}"#).expect("cannot deserialize");
    router.insert(route_1.into_route());

    router
}


#[test]
fn test_action_seo_override_meta_keywords_1() {
    let router = setup_action_seo_override_meta_keywords();
    let request = Request::new(r#"/source"#.to_string(),None,None,
    None).to_http_request().expect("");
    let matched = router.match_request(&request);

    assert_eq!(!matched.is_empty(), true);

    let action = Action::from_routes_rule(matched, &request);

    let body_filter = action.create_filter_body(0);
    assert_eq!(body_filter.is_some(), true);

    let mut filter = body_filter.unwrap();
    let mut new_body = filter.filter(r#"<html><head><meta /></head></html>"#.to_string());
    new_body.push_str(filter.end().as_str());
    assert_eq!(new_body, r#"<html><head><meta /><meta name="keywords" content="some, keywords, here" /></head></html>"#)
}

#[test]
fn test_action_seo_override_meta_keywords_2() {
    let router = setup_action_seo_override_meta_keywords();
    let request = Request::new(r#"/source"#.to_string(),None,None,
    None).to_http_request().expect("");
    let matched = router.match_request(&request);

    assert_eq!(!matched.is_empty(), true);

    let action = Action::from_routes_rule(matched, &request);

    let body_filter = action.create_filter_body(0);
    assert_eq!(body_filter.is_some(), true);

    let mut filter = body_filter.unwrap();
    let mut new_body = filter.filter(r#"<html><head><meta name="keywords" /></head></html>"#.to_string());
    new_body.push_str(filter.end().as_str());
    assert_eq!(new_body, r#"<html><head><meta name="keywords" content="some, keywords, here" /></head></html>"#)
}

#[test]
fn test_action_seo_override_meta_keywords_3() {
    let router = setup_action_seo_override_meta_keywords();
    let request = Request::new(r#"/source"#.to_string(),None,None,
    None).to_http_request().expect("");
    let matched = router.match_request(&request);

    assert_eq!(!matched.is_empty(), true);

    let action = Action::from_routes_rule(matched, &request);

    let body_filter = action.create_filter_body(0);
    assert_eq!(body_filter.is_some(), true);

    let mut filter = body_filter.unwrap();
    let mut new_body = filter.filter(r#"<html><head><meta name="keywords" content="these, were, old, keywords" /></head></html>"#.to_string());
    new_body.push_str(filter.end().as_str());
    assert_eq!(new_body, r#"<html><head><meta name="keywords" content="some, keywords, here" /></head></html>"#)
}

#[test]
fn test_action_seo_override_meta_keywords_4() {
    let router = setup_action_seo_override_meta_keywords();
    let request = Request::new(r#"/source"#.to_string(),None,None,
    None).to_http_request().expect("");
    let matched = router.match_request(&request);

    assert_eq!(!matched.is_empty(), true);

    let action = Action::from_routes_rule(matched, &request);

    let body_filter = action.create_filter_body(0);
    assert_eq!(body_filter.is_some(), true);

    let mut filter = body_filter.unwrap();
    let mut new_body = filter.filter(r#"<html><head><meta></head></html>"#.to_string());
    new_body.push_str(filter.end().as_str());
    assert_eq!(new_body, r#"<html><head><meta><meta name="keywords" content="some, keywords, here" /></head></html>"#)
}

#[test]
fn test_action_seo_override_meta_keywords_5() {
    let router = setup_action_seo_override_meta_keywords();
    let request = Request::new(r#"/source"#.to_string(),None,None,
    None).to_http_request().expect("");
    let matched = router.match_request(&request);

    assert_eq!(!matched.is_empty(), true);

    let action = Action::from_routes_rule(matched, &request);

    let body_filter = action.create_filter_body(0);
    assert_eq!(body_filter.is_some(), true);

    let mut filter = body_filter.unwrap();
    let mut new_body = filter.filter(r#"<html><head><meta name="keywords"></head></html>"#.to_string());
    new_body.push_str(filter.end().as_str());
    assert_eq!(new_body, r#"<html><head><meta name="keywords" content="some, keywords, here" /></head></html>"#)
}

#[test]
fn test_action_seo_override_meta_keywords_6() {
    let router = setup_action_seo_override_meta_keywords();
    let request = Request::new(r#"/source"#.to_string(),None,None,
    None).to_http_request().expect("");
    let matched = router.match_request(&request);

    assert_eq!(!matched.is_empty(), true);

    let action = Action::from_routes_rule(matched, &request);

    let body_filter = action.create_filter_body(0);
    assert_eq!(body_filter.is_some(), true);

    let mut filter = body_filter.unwrap();
    let mut new_body = filter.filter(r#"<html><head><meta name="keywords" content="these, were, old, keywords"></head></html>"#.to_string());
    new_body.push_str(filter.end().as_str());
    assert_eq!(new_body, r#"<html><head><meta name="keywords" content="some, keywords, here" /></head></html>"#)
}


fn setup_action_seo_override_og_description() -> Router<Rule> {
    let mut router = Router::<Rule>::default();

    let route_1: Rule = serde_json::from_str(r#"{"body_filters":[{"action":"append_child","css_selector":"meta[property=\"og:description\"]","element_tree":["html","head"],"value":"<meta property=\"og:description\" content=\"🍕🍕 Pizza rapido 🍕🍕\" />"},{"action":"replace","css_selector":"meta[property=\"og:description\"]","element_tree":["html","head","meta"],"value":"<meta property=\"og:description\" content=\"🍕🍕 Pizza rapido 🍕🍕\" />"}],"id":"override-og-description-emoji-rule","markers":null,"rank":0,"redirect_code":null,"source":{"host":"","path":"/pizza-rapido","query":""},"target":null}"#).expect("cannot deserialize");
    router.insert(route_1.into_route());

    let route_2: Rule = serde_json::from_str(r#"{"body_filters":[{"action":"append_child","css_selector":"meta[property=\"og:description\"]","element_tree":["html","head"],"value":"<meta property=\"og:description\" content=\"New Description\" />"},{"action":"replace","css_selector":"meta[property=\"og:description\"]","element_tree":["html","head","meta"],"value":"<meta property=\"og:description\" content=\"New Description\" />"}],"id":"override-og-description-rule","markers":null,"rank":0,"redirect_code":null,"source":{"host":"","path":"/source","query":""},"target":null}"#).expect("cannot deserialize");
    router.insert(route_2.into_route());

    router
}


#[test]
fn test_action_seo_override_og_description_1() {
    let router = setup_action_seo_override_og_description();
    let request = Request::new(r#"/source"#.to_string(),None,None,
    None).to_http_request().expect("");
    let matched = router.match_request(&request);

    assert_eq!(!matched.is_empty(), true);

    let action = Action::from_routes_rule(matched, &request);

    let body_filter = action.create_filter_body(0);
    assert_eq!(body_filter.is_some(), true);

    let mut filter = body_filter.unwrap();
    let mut new_body = filter.filter(r#"<html><head><description>Old description</description><meta /></head></html>"#.to_string());
    new_body.push_str(filter.end().as_str());
    assert_eq!(new_body, r#"<html><head><description>Old description</description><meta /><meta property="og:description" content="New Description" /></head></html>"#)
}

#[test]
fn test_action_seo_override_og_description_2() {
    let router = setup_action_seo_override_og_description();
    let request = Request::new(r#"/source"#.to_string(),None,None,
    None).to_http_request().expect("");
    let matched = router.match_request(&request);

    assert_eq!(!matched.is_empty(), true);

    let action = Action::from_routes_rule(matched, &request);

    let body_filter = action.create_filter_body(0);
    assert_eq!(body_filter.is_some(), true);

    let mut filter = body_filter.unwrap();
    let mut new_body = filter.filter(r#"<html><head><meta /></head></html>"#.to_string());
    new_body.push_str(filter.end().as_str());
    assert_eq!(new_body, r#"<html><head><meta /><meta property="og:description" content="New Description" /></head></html>"#)
}

#[test]
fn test_action_seo_override_og_description_3() {
    let router = setup_action_seo_override_og_description();
    let request = Request::new(r#"/source"#.to_string(),None,None,
    None).to_http_request().expect("");
    let matched = router.match_request(&request);

    assert_eq!(!matched.is_empty(), true);

    let action = Action::from_routes_rule(matched, &request);

    let body_filter = action.create_filter_body(0);
    assert_eq!(body_filter.is_some(), true);

    let mut filter = body_filter.unwrap();
    let mut new_body = filter.filter(r#"<html><head><description>Old description</description><meta /><meta property="og:description" content="Old Description" /></head></html>"#.to_string());
    new_body.push_str(filter.end().as_str());
    assert_eq!(new_body, r#"<html><head><description>Old description</description><meta /><meta property="og:description" content="New Description" /></head></html>"#)
}

#[test]
fn test_action_seo_override_og_description_4() {
    let router = setup_action_seo_override_og_description();
    let request = Request::new(r#"/source"#.to_string(),None,None,
    None).to_http_request().expect("");
    let matched = router.match_request(&request);

    assert_eq!(!matched.is_empty(), true);

    let action = Action::from_routes_rule(matched, &request);

    let body_filter = action.create_filter_body(0);
    assert_eq!(body_filter.is_some(), true);

    let mut filter = body_filter.unwrap();
    let mut new_body = filter.filter(r#"<html><head><description>Old description</description><meta><meta property="og:description" content="Old Description" /></head></html>"#.to_string());
    new_body.push_str(filter.end().as_str());
    assert_eq!(new_body, r#"<html><head><description>Old description</description><meta><meta property="og:description" content="New Description" /></head></html>"#)
}

#[test]
fn test_action_seo_override_og_description_5() {
    let router = setup_action_seo_override_og_description();
    let request = Request::new(r#"/source"#.to_string(),None,None,
    None).to_http_request().expect("");
    let matched = router.match_request(&request);

    assert_eq!(!matched.is_empty(), true);

    let action = Action::from_routes_rule(matched, &request);

    let body_filter = action.create_filter_body(0);
    assert_eq!(body_filter.is_some(), true);

    let mut filter = body_filter.unwrap();
    let mut new_body = filter.filter(r#"<html><head><description>Old description</description><meta></head></html>"#.to_string());
    new_body.push_str(filter.end().as_str());
    assert_eq!(new_body, r#"<html><head><description>Old description</description><meta><meta property="og:description" content="New Description" /></head></html>"#)
}

#[test]
fn test_action_seo_override_og_description_6() {
    let router = setup_action_seo_override_og_description();
    let request = Request::new(r#"/source"#.to_string(),None,None,
    None).to_http_request().expect("");
    let matched = router.match_request(&request);

    assert_eq!(!matched.is_empty(), true);

    let action = Action::from_routes_rule(matched, &request);

    let body_filter = action.create_filter_body(0);
    assert_eq!(body_filter.is_some(), true);

    let mut filter = body_filter.unwrap();
    let mut new_body = filter.filter(r#"<html><head><description>Old description</description><meta property="no-closing"><meta property="og:description" content="Old Description" /></head></html>"#.to_string());
    new_body.push_str(filter.end().as_str());
    assert_eq!(new_body, r#"<html><head><description>Old description</description><meta property="no-closing"><meta property="og:description" content="New Description" /></head></html>"#)
}

#[test]
fn test_action_seo_override_og_description_7() {
    let router = setup_action_seo_override_og_description();
    let request = Request::new(r#"/pizza-rapido"#.to_string(),None,None,
    None).to_http_request().expect("");
    let matched = router.match_request(&request);

    assert_eq!(!matched.is_empty(), true);

    let action = Action::from_routes_rule(matched, &request);

    let body_filter = action.create_filter_body(0);
    assert_eq!(body_filter.is_some(), true);

    let mut filter = body_filter.unwrap();
    let mut new_body = filter.filter(r#"<html><head><meta property="og:description" content="Pizza rapido" /></head></html>"#.to_string());
    new_body.push_str(filter.end().as_str());
    assert_eq!(new_body, r#"<html><head><meta property="og:description" content="🍕🍕 Pizza rapido 🍕🍕" /></head></html>"#)
}


fn setup_action_seo_override_og_image() -> Router<Rule> {
    let mut router = Router::<Rule>::default();

    let route_1: Rule = serde_json::from_str(r#"{"body_filters":[{"action":"append_child","css_selector":"meta[property=\"og:image\"]","element_tree":["html","head"],"value":"<meta property=\"og:image\" content=\"/some-image.png\" />"},{"action":"replace","css_selector":"meta[property=\"og:image\"]","element_tree":["html","head","meta"],"value":"<meta property=\"og:image\" content=\"/some-image.png\" />"}],"id":"override-og-image-rule","markers":null,"rank":0,"redirect_code":null,"source":{"host":"","path":"/source","query":""},"target":null}"#).expect("cannot deserialize");
    router.insert(route_1.into_route());

    router
}


#[test]
fn test_action_seo_override_og_image_1() {
    let router = setup_action_seo_override_og_image();
    let request = Request::new(r#"/source"#.to_string(),None,None,
    None).to_http_request().expect("");
    let matched = router.match_request(&request);

    assert_eq!(!matched.is_empty(), true);

    let action = Action::from_routes_rule(matched, &request);

    let body_filter = action.create_filter_body(0);
    assert_eq!(body_filter.is_some(), true);

    let mut filter = body_filter.unwrap();
    let mut new_body = filter.filter(r#"<html><head><meta /></head></html>"#.to_string());
    new_body.push_str(filter.end().as_str());
    assert_eq!(new_body, r#"<html><head><meta /><meta property="og:image" content="/some-image.png" /></head></html>"#)
}

#[test]
fn test_action_seo_override_og_image_2() {
    let router = setup_action_seo_override_og_image();
    let request = Request::new(r#"/source"#.to_string(),None,None,
    None).to_http_request().expect("");
    let matched = router.match_request(&request);

    assert_eq!(!matched.is_empty(), true);

    let action = Action::from_routes_rule(matched, &request);

    let body_filter = action.create_filter_body(0);
    assert_eq!(body_filter.is_some(), true);

    let mut filter = body_filter.unwrap();
    let mut new_body = filter.filter(r#"<html><head><meta></head></html>"#.to_string());
    new_body.push_str(filter.end().as_str());
    assert_eq!(new_body, r#"<html><head><meta><meta property="og:image" content="/some-image.png" /></head></html>"#)
}

#[test]
fn test_action_seo_override_og_image_3() {
    let router = setup_action_seo_override_og_image();
    let request = Request::new(r#"/source"#.to_string(),None,None,
    None).to_http_request().expect("");
    let matched = router.match_request(&request);

    assert_eq!(!matched.is_empty(), true);

    let action = Action::from_routes_rule(matched, &request);

    let body_filter = action.create_filter_body(0);
    assert_eq!(body_filter.is_some(), true);

    let mut filter = body_filter.unwrap();
    let mut new_body = filter.filter(r#"<html><head><meta /><meta property="og:image" content="/old-image.png" /></head></html>"#.to_string());
    new_body.push_str(filter.end().as_str());
    assert_eq!(new_body, r#"<html><head><meta /><meta property="og:image" content="/some-image.png" /></head></html>"#)
}

#[test]
fn test_action_seo_override_og_image_4() {
    let router = setup_action_seo_override_og_image();
    let request = Request::new(r#"/source"#.to_string(),None,None,
    None).to_http_request().expect("");
    let matched = router.match_request(&request);

    assert_eq!(!matched.is_empty(), true);

    let action = Action::from_routes_rule(matched, &request);

    let body_filter = action.create_filter_body(0);
    assert_eq!(body_filter.is_some(), true);

    let mut filter = body_filter.unwrap();
    let mut new_body = filter.filter(r#"<html><head><meta property="no-closing"><meta property="og:image" content="/old-image.png" /></head></html>"#.to_string());
    new_body.push_str(filter.end().as_str());
    assert_eq!(new_body, r#"<html><head><meta property="no-closing"><meta property="og:image" content="/some-image.png" /></head></html>"#)
}


fn setup_action_seo_override_og_locale() -> Router<Rule> {
    let mut router = Router::<Rule>::default();

    let route_1: Rule = serde_json::from_str(r#"{"body_filters":[{"action":"append_child","css_selector":"meta[property=\"og:locale\"]","element_tree":["html","head"],"value":"<meta property=\"og:locale\" content=\"fr_FR\" />"},{"action":"replace","css_selector":"meta[property=\"og:locale\"]","element_tree":["html","head","meta"],"value":"<meta property=\"og:locale\" content=\"fr_FR\" />"}],"id":"override-og-locale-rule","markers":null,"rank":0,"redirect_code":null,"source":{"host":"","path":"/source","query":""},"target":null}"#).expect("cannot deserialize");
    router.insert(route_1.into_route());

    router
}


#[test]
fn test_action_seo_override_og_locale_1() {
    let router = setup_action_seo_override_og_locale();
    let request = Request::new(r#"/source"#.to_string(),None,None,
    None).to_http_request().expect("");
    let matched = router.match_request(&request);

    assert_eq!(!matched.is_empty(), true);

    let action = Action::from_routes_rule(matched, &request);

    let body_filter = action.create_filter_body(0);
    assert_eq!(body_filter.is_some(), true);

    let mut filter = body_filter.unwrap();
    let mut new_body = filter.filter(r#"<html><head><meta /></head></html>"#.to_string());
    new_body.push_str(filter.end().as_str());
    assert_eq!(new_body, r#"<html><head><meta /><meta property="og:locale" content="fr_FR" /></head></html>"#)
}

#[test]
fn test_action_seo_override_og_locale_2() {
    let router = setup_action_seo_override_og_locale();
    let request = Request::new(r#"/source"#.to_string(),None,None,
    None).to_http_request().expect("");
    let matched = router.match_request(&request);

    assert_eq!(!matched.is_empty(), true);

    let action = Action::from_routes_rule(matched, &request);

    let body_filter = action.create_filter_body(0);
    assert_eq!(body_filter.is_some(), true);

    let mut filter = body_filter.unwrap();
    let mut new_body = filter.filter(r#"<html><head><meta></head></html>"#.to_string());
    new_body.push_str(filter.end().as_str());
    assert_eq!(new_body, r#"<html><head><meta><meta property="og:locale" content="fr_FR" /></head></html>"#)
}

#[test]
fn test_action_seo_override_og_locale_3() {
    let router = setup_action_seo_override_og_locale();
    let request = Request::new(r#"/source"#.to_string(),None,None,
    None).to_http_request().expect("");
    let matched = router.match_request(&request);

    assert_eq!(!matched.is_empty(), true);

    let action = Action::from_routes_rule(matched, &request);

    let body_filter = action.create_filter_body(0);
    assert_eq!(body_filter.is_some(), true);

    let mut filter = body_filter.unwrap();
    let mut new_body = filter.filter(r#"<html><head><meta /><meta property="og:locale" content="en_GB" /></head></html>"#.to_string());
    new_body.push_str(filter.end().as_str());
    assert_eq!(new_body, r#"<html><head><meta /><meta property="og:locale" content="fr_FR" /></head></html>"#)
}

#[test]
fn test_action_seo_override_og_locale_4() {
    let router = setup_action_seo_override_og_locale();
    let request = Request::new(r#"/source"#.to_string(),None,None,
    None).to_http_request().expect("");
    let matched = router.match_request(&request);

    assert_eq!(!matched.is_empty(), true);

    let action = Action::from_routes_rule(matched, &request);

    let body_filter = action.create_filter_body(0);
    assert_eq!(body_filter.is_some(), true);

    let mut filter = body_filter.unwrap();
    let mut new_body = filter.filter(r#"<html><head><meta property="no-closing"><meta property="og:locale" content="en_GB" /></head></html>"#.to_string());
    new_body.push_str(filter.end().as_str());
    assert_eq!(new_body, r#"<html><head><meta property="no-closing"><meta property="og:locale" content="fr_FR" /></head></html>"#)
}


fn setup_action_seo_override_og_site_name() -> Router<Rule> {
    let mut router = Router::<Rule>::default();

    let route_1: Rule = serde_json::from_str(r#"{"body_filters":[{"action":"append_child","css_selector":"meta[property=\"og:site_name\"]","element_tree":["html","head"],"value":"<meta property=\"og:site_name\" content=\"redirection.io\" />"},{"action":"replace","css_selector":"meta[property=\"og:site_name\"]","element_tree":["html","head","meta"],"value":"<meta property=\"og:site_name\" content=\"redirection.io\" />"}],"id":"override-og-site_name-rule","markers":null,"rank":0,"redirect_code":null,"source":{"host":"","path":"/source","query":""},"target":null}"#).expect("cannot deserialize");
    router.insert(route_1.into_route());

    router
}


#[test]
fn test_action_seo_override_og_site_name_1() {
    let router = setup_action_seo_override_og_site_name();
    let request = Request::new(r#"/source"#.to_string(),None,None,
    None).to_http_request().expect("");
    let matched = router.match_request(&request);

    assert_eq!(!matched.is_empty(), true);

    let action = Action::from_routes_rule(matched, &request);

    let body_filter = action.create_filter_body(0);
    assert_eq!(body_filter.is_some(), true);

    let mut filter = body_filter.unwrap();
    let mut new_body = filter.filter(r#"<html><head><meta /></head></html>"#.to_string());
    new_body.push_str(filter.end().as_str());
    assert_eq!(new_body, r#"<html><head><meta /><meta property="og:site_name" content="redirection.io" /></head></html>"#)
}

#[test]
fn test_action_seo_override_og_site_name_2() {
    let router = setup_action_seo_override_og_site_name();
    let request = Request::new(r#"/source"#.to_string(),None,None,
    None).to_http_request().expect("");
    let matched = router.match_request(&request);

    assert_eq!(!matched.is_empty(), true);

    let action = Action::from_routes_rule(matched, &request);

    let body_filter = action.create_filter_body(0);
    assert_eq!(body_filter.is_some(), true);

    let mut filter = body_filter.unwrap();
    let mut new_body = filter.filter(r#"<html><head><meta></head></html>"#.to_string());
    new_body.push_str(filter.end().as_str());
    assert_eq!(new_body, r#"<html><head><meta><meta property="og:site_name" content="redirection.io" /></head></html>"#)
}

#[test]
fn test_action_seo_override_og_site_name_3() {
    let router = setup_action_seo_override_og_site_name();
    let request = Request::new(r#"/source"#.to_string(),None,None,
    None).to_http_request().expect("");
    let matched = router.match_request(&request);

    assert_eq!(!matched.is_empty(), true);

    let action = Action::from_routes_rule(matched, &request);

    let body_filter = action.create_filter_body(0);
    assert_eq!(body_filter.is_some(), true);

    let mut filter = body_filter.unwrap();
    let mut new_body = filter.filter(r#"<html><head><meta /><meta property="og:site_name" content="JoliCode" /></head></html>"#.to_string());
    new_body.push_str(filter.end().as_str());
    assert_eq!(new_body, r#"<html><head><meta /><meta property="og:site_name" content="redirection.io" /></head></html>"#)
}

#[test]
fn test_action_seo_override_og_site_name_4() {
    let router = setup_action_seo_override_og_site_name();
    let request = Request::new(r#"/source"#.to_string(),None,None,
    None).to_http_request().expect("");
    let matched = router.match_request(&request);

    assert_eq!(!matched.is_empty(), true);

    let action = Action::from_routes_rule(matched, &request);

    let body_filter = action.create_filter_body(0);
    assert_eq!(body_filter.is_some(), true);

    let mut filter = body_filter.unwrap();
    let mut new_body = filter.filter(r#"<html><head><meta property="no-closing"><meta property="og:site_name" content="JoliCode" /></head></html>"#.to_string());
    new_body.push_str(filter.end().as_str());
    assert_eq!(new_body, r#"<html><head><meta property="no-closing"><meta property="og:site_name" content="redirection.io" /></head></html>"#)
}


fn setup_action_seo_override_og_title() -> Router<Rule> {
    let mut router = Router::<Rule>::default();

    let route_1: Rule = serde_json::from_str(r#"{"body_filters":[{"action":"append_child","css_selector":"meta[property=\"og:title\"]","element_tree":["html","head"],"value":"<meta property=\"og:title\" content=\"New Title\" />"},{"action":"replace","css_selector":"meta[property=\"og:title\"]","element_tree":["html","head","meta"],"value":"<meta property=\"og:title\" content=\"New Title\" />"}],"id":"override-og-title-rule","markers":null,"rank":0,"redirect_code":null,"source":{"host":"","path":"/source","query":""},"target":null}"#).expect("cannot deserialize");
    router.insert(route_1.into_route());

    router
}


#[test]
fn test_action_seo_override_og_title_1() {
    let router = setup_action_seo_override_og_title();
    let request = Request::new(r#"/source"#.to_string(),None,None,
    None).to_http_request().expect("");
    let matched = router.match_request(&request);

    assert_eq!(!matched.is_empty(), true);

    let action = Action::from_routes_rule(matched, &request);

    let body_filter = action.create_filter_body(0);
    assert_eq!(body_filter.is_some(), true);

    let mut filter = body_filter.unwrap();
    let mut new_body = filter.filter(r#"<html><head><title>Old title</title><meta /></head></html>"#.to_string());
    new_body.push_str(filter.end().as_str());
    assert_eq!(new_body, r#"<html><head><title>Old title</title><meta /><meta property="og:title" content="New Title" /></head></html>"#)
}

#[test]
fn test_action_seo_override_og_title_2() {
    let router = setup_action_seo_override_og_title();
    let request = Request::new(r#"/source"#.to_string(),None,None,
    None).to_http_request().expect("");
    let matched = router.match_request(&request);

    assert_eq!(!matched.is_empty(), true);

    let action = Action::from_routes_rule(matched, &request);

    let body_filter = action.create_filter_body(0);
    assert_eq!(body_filter.is_some(), true);

    let mut filter = body_filter.unwrap();
    let mut new_body = filter.filter(r#"<html><head><meta /></head></html>"#.to_string());
    new_body.push_str(filter.end().as_str());
    assert_eq!(new_body, r#"<html><head><meta /><meta property="og:title" content="New Title" /></head></html>"#)
}

#[test]
fn test_action_seo_override_og_title_3() {
    let router = setup_action_seo_override_og_title();
    let request = Request::new(r#"/source"#.to_string(),None,None,
    None).to_http_request().expect("");
    let matched = router.match_request(&request);

    assert_eq!(!matched.is_empty(), true);

    let action = Action::from_routes_rule(matched, &request);

    let body_filter = action.create_filter_body(0);
    assert_eq!(body_filter.is_some(), true);

    let mut filter = body_filter.unwrap();
    let mut new_body = filter.filter(r#"<html><head><title>Old title</title><meta /><meta property="og:title" content="Old Title" /></head></html>"#.to_string());
    new_body.push_str(filter.end().as_str());
    assert_eq!(new_body, r#"<html><head><title>Old title</title><meta /><meta property="og:title" content="New Title" /></head></html>"#)
}

#[test]
fn test_action_seo_override_og_title_4() {
    let router = setup_action_seo_override_og_title();
    let request = Request::new(r#"/source"#.to_string(),None,None,
    None).to_http_request().expect("");
    let matched = router.match_request(&request);

    assert_eq!(!matched.is_empty(), true);

    let action = Action::from_routes_rule(matched, &request);

    let body_filter = action.create_filter_body(0);
    assert_eq!(body_filter.is_some(), true);

    let mut filter = body_filter.unwrap();
    let mut new_body = filter.filter(r#"<html><head><title>Old title</title><meta property="no-closing"><meta property="og:title" content="Old Title" /></head></html>"#.to_string());
    new_body.push_str(filter.end().as_str());
    assert_eq!(new_body, r#"<html><head><title>Old title</title><meta property="no-closing"><meta property="og:title" content="New Title" /></head></html>"#)
}


fn setup_action_seo_override_og_type() -> Router<Rule> {
    let mut router = Router::<Rule>::default();

    let route_1: Rule = serde_json::from_str(r#"{"body_filters":[{"action":"append_child","css_selector":"meta[property=\"og:type\"]","element_tree":["html","head"],"value":"<meta property=\"og:type\" content=\"website\" />"},{"action":"replace","css_selector":"meta[property=\"og:type\"]","element_tree":["html","head","meta"],"value":"<meta property=\"og:type\" content=\"website\" />"}],"id":"override-og-type-rule","markers":null,"rank":0,"redirect_code":null,"source":{"host":"","path":"/source","query":""},"target":null}"#).expect("cannot deserialize");
    router.insert(route_1.into_route());

    router
}


#[test]
fn test_action_seo_override_og_type_1() {
    let router = setup_action_seo_override_og_type();
    let request = Request::new(r#"/source"#.to_string(),None,None,
    None).to_http_request().expect("");
    let matched = router.match_request(&request);

    assert_eq!(!matched.is_empty(), true);

    let action = Action::from_routes_rule(matched, &request);

    let body_filter = action.create_filter_body(0);
    assert_eq!(body_filter.is_some(), true);

    let mut filter = body_filter.unwrap();
    let mut new_body = filter.filter(r#"<html><head><meta /></head></html>"#.to_string());
    new_body.push_str(filter.end().as_str());
    assert_eq!(new_body, r#"<html><head><meta /><meta property="og:type" content="website" /></head></html>"#)
}

#[test]
fn test_action_seo_override_og_type_2() {
    let router = setup_action_seo_override_og_type();
    let request = Request::new(r#"/source"#.to_string(),None,None,
    None).to_http_request().expect("");
    let matched = router.match_request(&request);

    assert_eq!(!matched.is_empty(), true);

    let action = Action::from_routes_rule(matched, &request);

    let body_filter = action.create_filter_body(0);
    assert_eq!(body_filter.is_some(), true);

    let mut filter = body_filter.unwrap();
    let mut new_body = filter.filter(r#"<html><head><meta /></head></html>"#.to_string());
    new_body.push_str(filter.end().as_str());
    assert_eq!(new_body, r#"<html><head><meta /><meta property="og:type" content="website" /></head></html>"#)
}

#[test]
fn test_action_seo_override_og_type_3() {
    let router = setup_action_seo_override_og_type();
    let request = Request::new(r#"/source"#.to_string(),None,None,
    None).to_http_request().expect("");
    let matched = router.match_request(&request);

    assert_eq!(!matched.is_empty(), true);

    let action = Action::from_routes_rule(matched, &request);

    let body_filter = action.create_filter_body(0);
    assert_eq!(body_filter.is_some(), true);

    let mut filter = body_filter.unwrap();
    let mut new_body = filter.filter(r#"<html><head><meta /><meta property="og:type" content="article" /></head></html>"#.to_string());
    new_body.push_str(filter.end().as_str());
    assert_eq!(new_body, r#"<html><head><meta /><meta property="og:type" content="website" /></head></html>"#)
}

#[test]
fn test_action_seo_override_og_type_4() {
    let router = setup_action_seo_override_og_type();
    let request = Request::new(r#"/source"#.to_string(),None,None,
    None).to_http_request().expect("");
    let matched = router.match_request(&request);

    assert_eq!(!matched.is_empty(), true);

    let action = Action::from_routes_rule(matched, &request);

    let body_filter = action.create_filter_body(0);
    assert_eq!(body_filter.is_some(), true);

    let mut filter = body_filter.unwrap();
    let mut new_body = filter.filter(r#"<html><head><meta property="no-closing"><meta property="og:type" content="article" /></head></html>"#.to_string());
    new_body.push_str(filter.end().as_str());
    assert_eq!(new_body, r#"<html><head><meta property="no-closing"><meta property="og:type" content="website" /></head></html>"#)
}


fn setup_action_seo_override_og_url() -> Router<Rule> {
    let mut router = Router::<Rule>::default();

    let route_1: Rule = serde_json::from_str(r#"{"body_filters":[{"action":"append_child","css_selector":"meta[property=\"og:url\"]","element_tree":["html","head"],"value":"<meta property=\"og:url\" content=\"https://redirection.io/features\" />"},{"action":"replace","css_selector":"meta[property=\"og:url\"]","element_tree":["html","head","meta"],"value":"<meta property=\"og:url\" content=\"https://redirection.io/features\" />"}],"id":"override-og-url-rule","markers":null,"rank":0,"redirect_code":null,"source":{"host":"","path":"/source","query":""},"target":null}"#).expect("cannot deserialize");
    router.insert(route_1.into_route());

    router
}


#[test]
fn test_action_seo_override_og_url_1() {
    let router = setup_action_seo_override_og_url();
    let request = Request::new(r#"/source"#.to_string(),None,None,
    None).to_http_request().expect("");
    let matched = router.match_request(&request);

    assert_eq!(!matched.is_empty(), true);

    let action = Action::from_routes_rule(matched, &request);

    let body_filter = action.create_filter_body(0);
    assert_eq!(body_filter.is_some(), true);

    let mut filter = body_filter.unwrap();
    let mut new_body = filter.filter(r#"<html><head><url>Old url</url><meta /></head></html>"#.to_string());
    new_body.push_str(filter.end().as_str());
    assert_eq!(new_body, r#"<html><head><url>Old url</url><meta /><meta property="og:url" content="https://redirection.io/features" /></head></html>"#)
}

#[test]
fn test_action_seo_override_og_url_2() {
    let router = setup_action_seo_override_og_url();
    let request = Request::new(r#"/source"#.to_string(),None,None,
    None).to_http_request().expect("");
    let matched = router.match_request(&request);

    assert_eq!(!matched.is_empty(), true);

    let action = Action::from_routes_rule(matched, &request);

    let body_filter = action.create_filter_body(0);
    assert_eq!(body_filter.is_some(), true);

    let mut filter = body_filter.unwrap();
    let mut new_body = filter.filter(r#"<html><head><meta /></head></html>"#.to_string());
    new_body.push_str(filter.end().as_str());
    assert_eq!(new_body, r#"<html><head><meta /><meta property="og:url" content="https://redirection.io/features" /></head></html>"#)
}

#[test]
fn test_action_seo_override_og_url_3() {
    let router = setup_action_seo_override_og_url();
    let request = Request::new(r#"/source"#.to_string(),None,None,
    None).to_http_request().expect("");
    let matched = router.match_request(&request);

    assert_eq!(!matched.is_empty(), true);

    let action = Action::from_routes_rule(matched, &request);

    let body_filter = action.create_filter_body(0);
    assert_eq!(body_filter.is_some(), true);

    let mut filter = body_filter.unwrap();
    let mut new_body = filter.filter(r#"<html><head><url>Old url</url><meta /><meta property="og:url" content="https://jolicode.com/" /></head></html>"#.to_string());
    new_body.push_str(filter.end().as_str());
    assert_eq!(new_body, r#"<html><head><url>Old url</url><meta /><meta property="og:url" content="https://redirection.io/features" /></head></html>"#)
}

#[test]
fn test_action_seo_override_og_url_4() {
    let router = setup_action_seo_override_og_url();
    let request = Request::new(r#"/source"#.to_string(),None,None,
    None).to_http_request().expect("");
    let matched = router.match_request(&request);

    assert_eq!(!matched.is_empty(), true);

    let action = Action::from_routes_rule(matched, &request);

    let body_filter = action.create_filter_body(0);
    assert_eq!(body_filter.is_some(), true);

    let mut filter = body_filter.unwrap();
    let mut new_body = filter.filter(r#"<html><head><url>Old url</url><meta property="no-closing"><meta property="og:url" content="https://jolicode.com/" /></head></html>"#.to_string());
    new_body.push_str(filter.end().as_str());
    assert_eq!(new_body, r#"<html><head><url>Old url</url><meta property="no-closing"><meta property="og:url" content="https://redirection.io/features" /></head></html>"#)
}


fn setup_action_seo_override_title() -> Router<Rule> {
    let mut router = Router::<Rule>::default();

    let route_1: Rule = serde_json::from_str(r#"{"body_filters":[{"action":"append_child","css_selector":"title","element_tree":["html","head"],"value":"<title>New Title</title>"},{"action":"replace","css_selector":"","element_tree":["html","head","title"],"value":"<title>New Title</title>"}],"id":"override-title-rule","markers":null,"rank":0,"redirect_code":null,"source":{"host":"","path":"/source","query":""},"target":null}"#).expect("cannot deserialize");
    router.insert(route_1.into_route());

    router
}


#[test]
fn test_action_seo_override_title_1() {
    let router = setup_action_seo_override_title();
    let request = Request::new(r#"/source"#.to_string(),None,None,
    None).to_http_request().expect("");
    let matched = router.match_request(&request);

    assert_eq!(!matched.is_empty(), true);

    let action = Action::from_routes_rule(matched, &request);

    let body_filter = action.create_filter_body(0);
    assert_eq!(body_filter.is_some(), true);

    let mut filter = body_filter.unwrap();
    let mut new_body = filter.filter(r#"<html><head><title>Old title</title><meta /></head></html>"#.to_string());
    new_body.push_str(filter.end().as_str());
    assert_eq!(new_body, r#"<html><head><title>New Title</title><meta /></head></html>"#)
}

#[test]
fn test_action_seo_override_title_2() {
    let router = setup_action_seo_override_title();
    let request = Request::new(r#"/source"#.to_string(),None,None,
    None).to_http_request().expect("");
    let matched = router.match_request(&request);

    assert_eq!(!matched.is_empty(), true);

    let action = Action::from_routes_rule(matched, &request);

    let body_filter = action.create_filter_body(0);
    assert_eq!(body_filter.is_some(), true);

    let mut filter = body_filter.unwrap();
    let mut new_body = filter.filter(r#"<html><head><meta /></head></html>"#.to_string());
    new_body.push_str(filter.end().as_str());
    assert_eq!(new_body, r#"<html><head><meta /><title>New Title</title></head></html>"#)
}

#[test]
fn test_action_seo_override_title_3() {
    let router = setup_action_seo_override_title();
    let request = Request::new(r#"/source"#.to_string(),None,None,
    None).to_http_request().expect("");
    let matched = router.match_request(&request);

    assert_eq!(!matched.is_empty(), true);

    let action = Action::from_routes_rule(matched, &request);

    let body_filter = action.create_filter_body(0);
    assert_eq!(body_filter.is_some(), true);

    let mut filter = body_filter.unwrap();
    let mut new_body = filter.filter(r#"<html><head><title>Old title</title><meta></head></html>"#.to_string());
    new_body.push_str(filter.end().as_str());
    assert_eq!(new_body, r#"<html><head><title>New Title</title><meta></head></html>"#)
}

#[test]
fn test_action_seo_override_title_4() {
    let router = setup_action_seo_override_title();
    let request = Request::new(r#"/source"#.to_string(),None,None,
    None).to_http_request().expect("");
    let matched = router.match_request(&request);

    assert_eq!(!matched.is_empty(), true);

    let action = Action::from_routes_rule(matched, &request);

    let body_filter = action.create_filter_body(0);
    assert_eq!(body_filter.is_some(), true);

    let mut filter = body_filter.unwrap();
    let mut new_body = filter.filter(r#"<html><head><meta></head></html>"#.to_string());
    new_body.push_str(filter.end().as_str());
    assert_eq!(new_body, r#"<html><head><meta><title>New Title</title></head></html>"#)
}


fn setup_marker() -> Router<Rule> {
    let mut router = Router::<Rule>::default();

    let route_1: Rule = serde_json::from_str(r#"{"body_filters":null,"id":"foobar-rule","markers":[{"name":"marker","regex":"(?:.+?)","transformers":null}],"rank":0,"redirect_code":302,"source":{"host":"","path":"/foo/@marker","query":""},"target":"/bar/@marker"}"#).expect("cannot deserialize");
    router.insert(route_1.into_route());

    let route_2: Rule = serde_json::from_str(r#"{"body_filters":null,"id":"rule-segfault-on-target","markers":[{"name":"marker","regex":"(?:([\\p{Ll}\\p{Lu}\\p{Lt}0-9]|%[0-9A-Z]{2})+?)","transformers":null}],"rank":0,"redirect_code":301,"source":{"host":"","path":"/monthly-tides/North%20Carolina-North%20Shore/@marker","query":""},"target":"https://www.usharbors.com/harbor/western-pacific-coast/@marker"}"#).expect("cannot deserialize");
    router.insert(route_2.into_route());

    let route_3: Rule = serde_json::from_str(r#"{"body_filters":null,"id":"transformerRule","markers":[{"name":"marker","regex":"(?:.+?)","transformers":[{"options":null,"type":"dasherize"},{"options":null,"type":"uppercase"}]}],"rank":0,"redirect_code":302,"source":{"host":"","path":"/a/@marker","query":""},"target":"/a/@marker"}"#).expect("cannot deserialize");
    router.insert(route_3.into_route());

    router
}


#[test]
fn test_marker_1() {
    let router = setup_marker();
    let request = Request::new(r#"/foo/test"#.to_string(),None,None,
    None).to_http_request().expect("");
    let matched = router.match_request(&request);

    assert_eq!(!matched.is_empty(), true);

    let action = Action::from_routes_rule(matched, &request);

    assert_eq!(action.get_status_code(0), 302);
    let headers = action.filter_headers(Vec::new(), 0);
    assert_eq!(headers.len(), 1);

    let target_header = headers.first().unwrap();
    assert_eq!(target_header.name, "Location");
    assert_eq!(target_header.value, r#"/bar/test"#);
}

#[test]
fn test_marker_2() {
    let router = setup_marker();
    let request = Request::new(r#"/foo2"#.to_string(),None,None,
    None).to_http_request().expect("");
    let matched = router.match_request(&request);

    assert_eq!(!matched.is_empty(), false);

}

#[test]
fn test_marker_3() {
    let router = setup_marker();
    let request = Request::new(r#"/a/test"#.to_string(),None,None,
    None).to_http_request().expect("");
    let matched = router.match_request(&request);

    assert_eq!(!matched.is_empty(), true);

    let action = Action::from_routes_rule(matched, &request);

    assert_eq!(action.get_status_code(0), 302);
    let headers = action.filter_headers(Vec::new(), 0);
    assert_eq!(headers.len(), 1);

    let target_header = headers.first().unwrap();
    assert_eq!(target_header.name, "Location");
    assert_eq!(target_header.value, r#"/a/TEST"#);
}

#[test]
fn test_marker_4() {
    let router = setup_marker();
    let request = Request::new(r#"/a/test_test"#.to_string(),None,None,
    None).to_http_request().expect("");
    let matched = router.match_request(&request);

    assert_eq!(!matched.is_empty(), true);

    let action = Action::from_routes_rule(matched, &request);

    assert_eq!(action.get_status_code(0), 302);
    let headers = action.filter_headers(Vec::new(), 0);
    assert_eq!(headers.len(), 1);

    let target_header = headers.first().unwrap();
    assert_eq!(target_header.name, "Location");
    assert_eq!(target_header.value, r#"/a/TEST-TEST"#);
}

#[test]
fn test_marker_5() {
    let router = setup_marker();
    let request = Request::new(r#"/monthly-tides/North%20Carolina-North%20Shore/test"#.to_string(),None,None,
    None).to_http_request().expect("");
    let matched = router.match_request(&request);

    assert_eq!(!matched.is_empty(), true);

    let action = Action::from_routes_rule(matched, &request);

    assert_eq!(action.get_status_code(0), 301);
    let headers = action.filter_headers(Vec::new(), 0);
    assert_eq!(headers.len(), 1);

    let target_header = headers.first().unwrap();
    assert_eq!(target_header.name, "Location");
    assert_eq!(target_header.value, r#"https://www.usharbors.com/harbor/western-pacific-coast/test"#);
}


fn setup_marker_in_querystring() -> Router<Rule> {
    let mut router = Router::<Rule>::default();

    let route_1: Rule = serde_json::from_str(r#"{"body_filters":null,"id":"matchany-rule","markers":[{"name":"marker","regex":"(?:.+?)","transformers":[]}],"rank":0,"redirect_code":302,"source":{"host":"","path":"/a@marker","query":""},"target":"/b@marker"}"#).expect("cannot deserialize");
    router.insert(route_1.into_route());

    let route_2: Rule = serde_json::from_str(r#"{"body_filters":null,"id":"querystring-rule","markers":[{"name":"marker","regex":"([\\p{Ll}])+?","transformers":[]}],"rank":0,"redirect_code":302,"source":{"host":"","path":"/querystring/from","query":"slug=@marker"},"target":"/querystring/target/some-target/@marker.html"}"#).expect("cannot deserialize");
    router.insert(route_2.into_route());

    router
}


#[test]
fn test_marker_in_querystring_1() {
    let router = setup_marker_in_querystring();
    let request = Request::new(r#"/querystring/from?slug=coucou"#.to_string(),None,None,
    None).to_http_request().expect("");
    let matched = router.match_request(&request);

    assert_eq!(!matched.is_empty(), true);

    let action = Action::from_routes_rule(matched, &request);

    assert_eq!(action.get_status_code(0), 302);
    let headers = action.filter_headers(Vec::new(), 0);
    assert_eq!(headers.len(), 1);

    let target_header = headers.first().unwrap();
    assert_eq!(target_header.name, "Location");
    assert_eq!(target_header.value, r#"/querystring/target/some-target/coucou.html"#);
}

#[test]
fn test_marker_in_querystring_2() {
    let router = setup_marker_in_querystring();
    let request = Request::new(r#"/querystring/from?slug=2048"#.to_string(),None,None,
    None).to_http_request().expect("");
    let matched = router.match_request(&request);

    assert_eq!(!matched.is_empty(), false);

}

#[test]
fn test_marker_in_querystring_3() {
    let router = setup_marker_in_querystring();
    let request = Request::new(r#"/querystring/from"#.to_string(),None,None,
    None).to_http_request().expect("");
    let matched = router.match_request(&request);

    assert_eq!(!matched.is_empty(), false);

}

#[test]
fn test_marker_in_querystring_4() {
    let router = setup_marker_in_querystring();
    let request = Request::new(r#"/a?yolo=yala"#.to_string(),None,None,
    None).to_http_request().expect("");
    let matched = router.match_request(&request);

    assert_eq!(!matched.is_empty(), true);

    let action = Action::from_routes_rule(matched, &request);

    assert_eq!(action.get_status_code(0), 302);
    let headers = action.filter_headers(Vec::new(), 0);
    assert_eq!(headers.len(), 1);

    let target_header = headers.first().unwrap();
    assert_eq!(target_header.name, "Location");
    assert_eq!(target_header.value, r#"/b?yolo=yala"#);
}


fn setup_marker_transformation_camelize() -> Router<Rule> {
    let mut router = Router::<Rule>::default();

    let route_1: Rule = serde_json::from_str(r#"{"body_filters":null,"id":"camelize-rule","markers":[{"name":"marker","regex":"([\\p{Ll}\\p{Lu}\\p{Lt}]|\\-)+?","transformers":[{"options":null,"type":"camelize"}]}],"rank":0,"redirect_code":302,"source":{"host":"","path":"/camelize/from/@marker","query":""},"target":"/camelize/target/@marker"}"#).expect("cannot deserialize");
    router.insert(route_1.into_route());

    router
}


#[test]
fn test_marker_transformation_camelize_1() {
    let router = setup_marker_transformation_camelize();
    let request = Request::new(r#"/camelize/from/helloPoney"#.to_string(),None,None,
    None).to_http_request().expect("");
    let matched = router.match_request(&request);

    assert_eq!(!matched.is_empty(), true);

    let action = Action::from_routes_rule(matched, &request);

    assert_eq!(action.get_status_code(0), 302);
    let headers = action.filter_headers(Vec::new(), 0);
    assert_eq!(headers.len(), 1);

    let target_header = headers.first().unwrap();
    assert_eq!(target_header.name, "Location");
    assert_eq!(target_header.value, r#"/camelize/target/helloPoney"#);
}

#[test]
fn test_marker_transformation_camelize_2() {
    let router = setup_marker_transformation_camelize();
    let request = Request::new(r#"/camelize/from/Hello-poney"#.to_string(),None,None,
    None).to_http_request().expect("");
    let matched = router.match_request(&request);

    assert_eq!(!matched.is_empty(), true);

    let action = Action::from_routes_rule(matched, &request);

    assert_eq!(action.get_status_code(0), 302);
    let headers = action.filter_headers(Vec::new(), 0);
    assert_eq!(headers.len(), 1);

    let target_header = headers.first().unwrap();
    assert_eq!(target_header.name, "Location");
    assert_eq!(target_header.value, r#"/camelize/target/helloPoney"#);
}

#[test]
fn test_marker_transformation_camelize_3() {
    let router = setup_marker_transformation_camelize();
    let request = Request::new(r#"/camelize/from/HelloPoney"#.to_string(),None,None,
    None).to_http_request().expect("");
    let matched = router.match_request(&request);

    assert_eq!(!matched.is_empty(), true);

    let action = Action::from_routes_rule(matched, &request);

    assert_eq!(action.get_status_code(0), 302);
    let headers = action.filter_headers(Vec::new(), 0);
    assert_eq!(headers.len(), 1);

    let target_header = headers.first().unwrap();
    assert_eq!(target_header.name, "Location");
    assert_eq!(target_header.value, r#"/camelize/target/helloPoney"#);
}

#[test]
fn test_marker_transformation_camelize_4() {
    let router = setup_marker_transformation_camelize();
    let request = Request::new(r#"/camelize/from/hello-pOney"#.to_string(),None,None,
    None).to_http_request().expect("");
    let matched = router.match_request(&request);

    assert_eq!(!matched.is_empty(), true);

    let action = Action::from_routes_rule(matched, &request);

    assert_eq!(action.get_status_code(0), 302);
    let headers = action.filter_headers(Vec::new(), 0);
    assert_eq!(headers.len(), 1);

    let target_header = headers.first().unwrap();
    assert_eq!(target_header.name, "Location");
    assert_eq!(target_header.value, r#"/camelize/target/helloPOney"#);
}


fn setup_marker_transformation_dasherize() -> Router<Rule> {
    let mut router = Router::<Rule>::default();

    let route_1: Rule = serde_json::from_str(r#"{"body_filters":null,"id":"dasherize-rule","markers":[{"name":"marker","regex":"([\\p{Ll}\\p{Lu}\\p{Lt}]|\\-)+?","transformers":[{"options":null,"type":"dasherize"}]}],"rank":0,"redirect_code":302,"source":{"host":"","path":"/dasherize/from/@marker","query":""},"target":"/dasherize/target/@marker"}"#).expect("cannot deserialize");
    router.insert(route_1.into_route());

    router
}


#[test]
fn test_marker_transformation_dasherize_1() {
    let router = setup_marker_transformation_dasherize();
    let request = Request::new(r#"/dasherize/from/HelloPoney"#.to_string(),None,None,
    None).to_http_request().expect("");
    let matched = router.match_request(&request);

    assert_eq!(!matched.is_empty(), true);

    let action = Action::from_routes_rule(matched, &request);

    assert_eq!(action.get_status_code(0), 302);
    let headers = action.filter_headers(Vec::new(), 0);
    assert_eq!(headers.len(), 1);

    let target_header = headers.first().unwrap();
    assert_eq!(target_header.name, "Location");
    assert_eq!(target_header.value, r#"/dasherize/target/hello-poney"#);
}

#[test]
fn test_marker_transformation_dasherize_2() {
    let router = setup_marker_transformation_dasherize();
    let request = Request::new(r#"/dasherize/from/helloPoney"#.to_string(),None,None,
    None).to_http_request().expect("");
    let matched = router.match_request(&request);

    assert_eq!(!matched.is_empty(), true);

    let action = Action::from_routes_rule(matched, &request);

    assert_eq!(action.get_status_code(0), 302);
    let headers = action.filter_headers(Vec::new(), 0);
    assert_eq!(headers.len(), 1);

    let target_header = headers.first().unwrap();
    assert_eq!(target_header.name, "Location");
    assert_eq!(target_header.value, r#"/dasherize/target/hello-poney"#);
}

#[test]
fn test_marker_transformation_dasherize_3() {
    let router = setup_marker_transformation_dasherize();
    let request = Request::new(r#"/dasherize/from/Hello-Poney"#.to_string(),None,None,
    None).to_http_request().expect("");
    let matched = router.match_request(&request);

    assert_eq!(!matched.is_empty(), true);

    let action = Action::from_routes_rule(matched, &request);

    assert_eq!(action.get_status_code(0), 302);
    let headers = action.filter_headers(Vec::new(), 0);
    assert_eq!(headers.len(), 1);

    let target_header = headers.first().unwrap();
    assert_eq!(target_header.name, "Location");
    assert_eq!(target_header.value, r#"/dasherize/target/hello-poney"#);
}


fn setup_marker_transformation_lowercase() -> Router<Rule> {
    let mut router = Router::<Rule>::default();

    let route_1: Rule = serde_json::from_str(r#"{"body_filters":null,"id":"lowercase-rule","markers":[{"name":"marker","regex":"([\\p{Ll}\\p{Lu}\\p{Lt}]|\\-)+?","transformers":[{"options":null,"type":"lowercase"}]}],"rank":0,"redirect_code":302,"source":{"host":"","path":"/lowercase/from/@marker","query":""},"target":"/lowercase/target/@marker"}"#).expect("cannot deserialize");
    router.insert(route_1.into_route());

    router
}


#[test]
fn test_marker_transformation_lowercase_1() {
    let router = setup_marker_transformation_lowercase();
    let request = Request::new(r#"/lowercase/from/HELLO-PONEY"#.to_string(),None,None,
    None).to_http_request().expect("");
    let matched = router.match_request(&request);

    assert_eq!(!matched.is_empty(), true);

    let action = Action::from_routes_rule(matched, &request);

    assert_eq!(action.get_status_code(0), 302);
    let headers = action.filter_headers(Vec::new(), 0);
    assert_eq!(headers.len(), 1);

    let target_header = headers.first().unwrap();
    assert_eq!(target_header.name, "Location");
    assert_eq!(target_header.value, r#"/lowercase/target/hello-poney"#);
}

#[test]
fn test_marker_transformation_lowercase_2() {
    let router = setup_marker_transformation_lowercase();
    let request = Request::new(r#"/lowercase/from/HeLlO-PoNeY"#.to_string(),None,None,
    None).to_http_request().expect("");
    let matched = router.match_request(&request);

    assert_eq!(!matched.is_empty(), true);

    let action = Action::from_routes_rule(matched, &request);

    assert_eq!(action.get_status_code(0), 302);
    let headers = action.filter_headers(Vec::new(), 0);
    assert_eq!(headers.len(), 1);

    let target_header = headers.first().unwrap();
    assert_eq!(target_header.name, "Location");
    assert_eq!(target_header.value, r#"/lowercase/target/hello-poney"#);
}

#[test]
fn test_marker_transformation_lowercase_3() {
    let router = setup_marker_transformation_lowercase();
    let request = Request::new(r#"/lowercase/from/hello-poney"#.to_string(),None,None,
    None).to_http_request().expect("");
    let matched = router.match_request(&request);

    assert_eq!(!matched.is_empty(), true);

    let action = Action::from_routes_rule(matched, &request);

    assert_eq!(action.get_status_code(0), 302);
    let headers = action.filter_headers(Vec::new(), 0);
    assert_eq!(headers.len(), 1);

    let target_header = headers.first().unwrap();
    assert_eq!(target_header.name, "Location");
    assert_eq!(target_header.value, r#"/lowercase/target/hello-poney"#);
}


fn setup_marker_transformation_replace() -> Router<Rule> {
    let mut router = Router::<Rule>::default();

    let route_1: Rule = serde_json::from_str(r#"{"body_filters":null,"id":"replace-rule","markers":[{"name":"marker","regex":"(cat|dog|fish)","transformers":[{"options":{"something":"cat","with":"tiger"},"type":"replace"}]}],"rank":0,"redirect_code":302,"source":{"host":"","path":"/replace/from/@marker","query":""},"target":"/replace/target/@marker"}"#).expect("cannot deserialize");
    router.insert(route_1.into_route());

    router
}


#[test]
fn test_marker_transformation_replace_1() {
    let router = setup_marker_transformation_replace();
    let request = Request::new(r#"/replace/from/poney"#.to_string(),None,None,
    None).to_http_request().expect("");
    let matched = router.match_request(&request);

    assert_eq!(!matched.is_empty(), false);

}

#[test]
fn test_marker_transformation_replace_2() {
    let router = setup_marker_transformation_replace();
    let request = Request::new(r#"/replace/from/cat"#.to_string(),None,None,
    None).to_http_request().expect("");
    let matched = router.match_request(&request);

    assert_eq!(!matched.is_empty(), true);

    let action = Action::from_routes_rule(matched, &request);

    assert_eq!(action.get_status_code(0), 302);
    let headers = action.filter_headers(Vec::new(), 0);
    assert_eq!(headers.len(), 1);

    let target_header = headers.first().unwrap();
    assert_eq!(target_header.name, "Location");
    assert_eq!(target_header.value, r#"/replace/target/tiger"#);
}

#[test]
fn test_marker_transformation_replace_3() {
    let router = setup_marker_transformation_replace();
    let request = Request::new(r#"/replace/from/dog"#.to_string(),None,None,
    None).to_http_request().expect("");
    let matched = router.match_request(&request);

    assert_eq!(!matched.is_empty(), true);

    let action = Action::from_routes_rule(matched, &request);

    assert_eq!(action.get_status_code(0), 302);
    let headers = action.filter_headers(Vec::new(), 0);
    assert_eq!(headers.len(), 1);

    let target_header = headers.first().unwrap();
    assert_eq!(target_header.name, "Location");
    assert_eq!(target_header.value, r#"/replace/target/dog"#);
}


fn setup_marker_transformation_slice() -> Router<Rule> {
    let mut router = Router::<Rule>::default();

    let route_1: Rule = serde_json::from_str(r#"{"body_filters":null,"id":"slice-middle-rule","markers":[{"name":"marker","regex":"([\\p{Lu}\\p{Lt}])+?","transformers":[{"options":{"from":"5","to":"15"},"type":"slice"}]}],"rank":0,"redirect_code":302,"source":{"host":"","path":"/slice-middle/from/@marker","query":""},"target":"/slice-middle/target/@marker"}"#).expect("cannot deserialize");
    router.insert(route_1.into_route());

    let route_2: Rule = serde_json::from_str(r#"{"body_filters":null,"id":"slice-rule","markers":[{"name":"marker","regex":"([\\p{Lu}\\p{Lt}])+?","transformers":[{"options":{"from":"0","to":"10"},"type":"slice"}]}],"rank":0,"redirect_code":302,"source":{"host":"","path":"/slice/from/@marker","query":""},"target":"/slice/target/@marker"}"#).expect("cannot deserialize");
    router.insert(route_2.into_route());

    router
}


#[test]
fn test_marker_transformation_slice_1() {
    let router = setup_marker_transformation_slice();
    let request = Request::new(r#"/slice/from/ABCDEFGHIJKLMNOPQRSTUVWXYZ"#.to_string(),None,None,
    None).to_http_request().expect("");
    let matched = router.match_request(&request);

    assert_eq!(!matched.is_empty(), true);

    let action = Action::from_routes_rule(matched, &request);

    assert_eq!(action.get_status_code(0), 302);
    let headers = action.filter_headers(Vec::new(), 0);
    assert_eq!(headers.len(), 1);

    let target_header = headers.first().unwrap();
    assert_eq!(target_header.name, "Location");
    assert_eq!(target_header.value, r#"/slice/target/ABCDEFGHIJ"#);
}

#[test]
fn test_marker_transformation_slice_2() {
    let router = setup_marker_transformation_slice();
    let request = Request::new(r#"/slice/from/ABCD"#.to_string(),None,None,
    None).to_http_request().expect("");
    let matched = router.match_request(&request);

    assert_eq!(!matched.is_empty(), true);

    let action = Action::from_routes_rule(matched, &request);

    assert_eq!(action.get_status_code(0), 302);
    let headers = action.filter_headers(Vec::new(), 0);
    assert_eq!(headers.len(), 1);

    let target_header = headers.first().unwrap();
    assert_eq!(target_header.name, "Location");
    assert_eq!(target_header.value, r#"/slice/target/ABCD"#);
}

#[test]
fn test_marker_transformation_slice_3() {
    let router = setup_marker_transformation_slice();
    let request = Request::new(r#"/slice-middle/from/ABCDEFGHIJKLMNOPQRSTUVWXYZ"#.to_string(),None,None,
    None).to_http_request().expect("");
    let matched = router.match_request(&request);

    assert_eq!(!matched.is_empty(), true);

    let action = Action::from_routes_rule(matched, &request);

    assert_eq!(action.get_status_code(0), 302);
    let headers = action.filter_headers(Vec::new(), 0);
    assert_eq!(headers.len(), 1);

    let target_header = headers.first().unwrap();
    assert_eq!(target_header.name, "Location");
    assert_eq!(target_header.value, r#"/slice-middle/target/FGHIJKLMNO"#);
}

#[test]
fn test_marker_transformation_slice_4() {
    let router = setup_marker_transformation_slice();
    let request = Request::new(r#"/slice-middle/from/ABCDEFGHIJ"#.to_string(),None,None,
    None).to_http_request().expect("");
    let matched = router.match_request(&request);

    assert_eq!(!matched.is_empty(), true);

    let action = Action::from_routes_rule(matched, &request);

    assert_eq!(action.get_status_code(0), 302);
    let headers = action.filter_headers(Vec::new(), 0);
    assert_eq!(headers.len(), 1);

    let target_header = headers.first().unwrap();
    assert_eq!(target_header.name, "Location");
    assert_eq!(target_header.value, r#"/slice-middle/target/FGHIJ"#);
}

#[test]
fn test_marker_transformation_slice_5() {
    let router = setup_marker_transformation_slice();
    let request = Request::new(r#"/slice-middle/from/ABCD"#.to_string(),None,None,
    None).to_http_request().expect("");
    let matched = router.match_request(&request);

    assert_eq!(!matched.is_empty(), true);

    let action = Action::from_routes_rule(matched, &request);

    assert_eq!(action.get_status_code(0), 302);
    let headers = action.filter_headers(Vec::new(), 0);
    assert_eq!(headers.len(), 1);

    let target_header = headers.first().unwrap();
    assert_eq!(target_header.name, "Location");
    assert_eq!(target_header.value, r#"/slice-middle/target/"#);
}


fn setup_marker_transformation_underscorize() -> Router<Rule> {
    let mut router = Router::<Rule>::default();

    let route_1: Rule = serde_json::from_str(r#"{"body_filters":null,"id":"underscorize-rule","markers":[{"name":"marker","regex":"([\\p{Ll}\\p{Lu}\\p{Lt}]|\\-|_)+?","transformers":[{"options":null,"type":"underscorize"}]}],"rank":0,"redirect_code":302,"source":{"host":"","path":"/underscorize/from/@marker","query":""},"target":"/underscorize/target/@marker"}"#).expect("cannot deserialize");
    router.insert(route_1.into_route());

    router
}


#[test]
fn test_marker_transformation_underscorize_1() {
    let router = setup_marker_transformation_underscorize();
    let request = Request::new(r#"/underscorize/from/hello_poney"#.to_string(),None,None,
    None).to_http_request().expect("");
    let matched = router.match_request(&request);

    assert_eq!(!matched.is_empty(), true);

    let action = Action::from_routes_rule(matched, &request);

    assert_eq!(action.get_status_code(0), 302);
    let headers = action.filter_headers(Vec::new(), 0);
    assert_eq!(headers.len(), 1);

    let target_header = headers.first().unwrap();
    assert_eq!(target_header.name, "Location");
    assert_eq!(target_header.value, r#"/underscorize/target/hello_poney"#);
}

#[test]
fn test_marker_transformation_underscorize_2() {
    let router = setup_marker_transformation_underscorize();
    let request = Request::new(r#"/underscorize/from/hello-poney"#.to_string(),None,None,
    None).to_http_request().expect("");
    let matched = router.match_request(&request);

    assert_eq!(!matched.is_empty(), true);

    let action = Action::from_routes_rule(matched, &request);

    assert_eq!(action.get_status_code(0), 302);
    let headers = action.filter_headers(Vec::new(), 0);
    assert_eq!(headers.len(), 1);

    let target_header = headers.first().unwrap();
    assert_eq!(target_header.name, "Location");
    assert_eq!(target_header.value, r#"/underscorize/target/hello_poney"#);
}

#[test]
fn test_marker_transformation_underscorize_3() {
    let router = setup_marker_transformation_underscorize();
    let request = Request::new(r#"/underscorize/from/HelloPoney"#.to_string(),None,None,
    None).to_http_request().expect("");
    let matched = router.match_request(&request);

    assert_eq!(!matched.is_empty(), true);

    let action = Action::from_routes_rule(matched, &request);

    assert_eq!(action.get_status_code(0), 302);
    let headers = action.filter_headers(Vec::new(), 0);
    assert_eq!(headers.len(), 1);

    let target_header = headers.first().unwrap();
    assert_eq!(target_header.name, "Location");
    assert_eq!(target_header.value, r#"/underscorize/target/hello_poney"#);
}

#[test]
fn test_marker_transformation_underscorize_4() {
    let router = setup_marker_transformation_underscorize();
    let request = Request::new(r#"/underscorize/from/helloPoney"#.to_string(),None,None,
    None).to_http_request().expect("");
    let matched = router.match_request(&request);

    assert_eq!(!matched.is_empty(), true);

    let action = Action::from_routes_rule(matched, &request);

    assert_eq!(action.get_status_code(0), 302);
    let headers = action.filter_headers(Vec::new(), 0);
    assert_eq!(headers.len(), 1);

    let target_header = headers.first().unwrap();
    assert_eq!(target_header.name, "Location");
    assert_eq!(target_header.value, r#"/underscorize/target/hello_poney"#);
}


fn setup_marker_transformation_uppercase() -> Router<Rule> {
    let mut router = Router::<Rule>::default();

    let route_1: Rule = serde_json::from_str(r#"{"body_filters":null,"id":"uppercase-rule","markers":[{"name":"marker","regex":"([\\p{Ll}\\p{Lu}\\p{Lt}]|\\-)+?","transformers":[{"options":null,"type":"uppercase"}]}],"rank":0,"redirect_code":302,"source":{"host":"","path":"/uppercase/from/@marker","query":""},"target":"/uppercase/target/@marker"}"#).expect("cannot deserialize");
    router.insert(route_1.into_route());

    router
}


#[test]
fn test_marker_transformation_uppercase_1() {
    let router = setup_marker_transformation_uppercase();
    let request = Request::new(r#"/uppercase/from/HELLO-PONEY"#.to_string(),None,None,
    None).to_http_request().expect("");
    let matched = router.match_request(&request);

    assert_eq!(!matched.is_empty(), true);

    let action = Action::from_routes_rule(matched, &request);

    assert_eq!(action.get_status_code(0), 302);
    let headers = action.filter_headers(Vec::new(), 0);
    assert_eq!(headers.len(), 1);

    let target_header = headers.first().unwrap();
    assert_eq!(target_header.name, "Location");
    assert_eq!(target_header.value, r#"/uppercase/target/HELLO-PONEY"#);
}

#[test]
fn test_marker_transformation_uppercase_2() {
    let router = setup_marker_transformation_uppercase();
    let request = Request::new(r#"/uppercase/from/HeLlO-PoNeY"#.to_string(),None,None,
    None).to_http_request().expect("");
    let matched = router.match_request(&request);

    assert_eq!(!matched.is_empty(), true);

    let action = Action::from_routes_rule(matched, &request);

    assert_eq!(action.get_status_code(0), 302);
    let headers = action.filter_headers(Vec::new(), 0);
    assert_eq!(headers.len(), 1);

    let target_header = headers.first().unwrap();
    assert_eq!(target_header.name, "Location");
    assert_eq!(target_header.value, r#"/uppercase/target/HELLO-PONEY"#);
}

#[test]
fn test_marker_transformation_uppercase_3() {
    let router = setup_marker_transformation_uppercase();
    let request = Request::new(r#"/uppercase/from/hello-poney"#.to_string(),None,None,
    None).to_http_request().expect("");
    let matched = router.match_request(&request);

    assert_eq!(!matched.is_empty(), true);

    let action = Action::from_routes_rule(matched, &request);

    assert_eq!(action.get_status_code(0), 302);
    let headers = action.filter_headers(Vec::new(), 0);
    assert_eq!(headers.len(), 1);

    let target_header = headers.first().unwrap();
    assert_eq!(target_header.name, "Location");
    assert_eq!(target_header.value, r#"/uppercase/target/HELLO-PONEY"#);
}


fn setup_marker_type_anything() -> Router<Rule> {
    let mut router = Router::<Rule>::default();

    let route_1: Rule = serde_json::from_str(r#"{"body_filters":null,"id":"anything-rule","markers":[{"name":"marker","regex":".*","transformers":null}],"rank":0,"redirect_code":302,"source":{"host":"","path":"/anything/from/@marker","query":""},"target":"/anything/target/@marker"}"#).expect("cannot deserialize");
    router.insert(route_1.into_route());

    router
}


#[test]
fn test_marker_type_anything_1() {
    let router = setup_marker_type_anything();
    let request = Request::new(r#"/anything/from/f6883ff9-f163-43d7-8177-bfa24277fd20"#.to_string(),None,None,
    None).to_http_request().expect("");
    let matched = router.match_request(&request);

    assert_eq!(!matched.is_empty(), true);

    let action = Action::from_routes_rule(matched, &request);

    assert_eq!(action.get_status_code(0), 302);
    let headers = action.filter_headers(Vec::new(), 0);
    assert_eq!(headers.len(), 1);

    let target_header = headers.first().unwrap();
    assert_eq!(target_header.name, "Location");
    assert_eq!(target_header.value, r#"/anything/target/f6883ff9-f163-43d7-8177-bfa24277fd20"#);
}

#[test]
fn test_marker_type_anything_2() {
    let router = setup_marker_type_anything();
    let request = Request::new(r#"/anything/from/HELLO"#.to_string(),None,None,
    None).to_http_request().expect("");
    let matched = router.match_request(&request);

    assert_eq!(!matched.is_empty(), true);

    let action = Action::from_routes_rule(matched, &request);

    assert_eq!(action.get_status_code(0), 302);
    let headers = action.filter_headers(Vec::new(), 0);
    assert_eq!(headers.len(), 1);

    let target_header = headers.first().unwrap();
    assert_eq!(target_header.name, "Location");
    assert_eq!(target_header.value, r#"/anything/target/HELLO"#);
}

#[test]
fn test_marker_type_anything_3() {
    let router = setup_marker_type_anything();
    let request = Request::new(r#"/anything/from/🤘"#.to_string(),None,None,
    None).to_http_request().expect("");
    let matched = router.match_request(&request);

    assert_eq!(!matched.is_empty(), true);

    let action = Action::from_routes_rule(matched, &request);

    assert_eq!(action.get_status_code(0), 302);
    let headers = action.filter_headers(Vec::new(), 0);
    assert_eq!(headers.len(), 1);

    let target_header = headers.first().unwrap();
    assert_eq!(target_header.name, "Location");
    assert_eq!(target_header.value, r#"/anything/target/%F0%9F%A4%98"#);
}


fn setup_marker_type_date() -> Router<Rule> {
    let mut router = Router::<Rule>::default();

    let route_1: Rule = serde_json::from_str(r#"{"body_filters":null,"id":"date-rule","markers":[{"name":"marker","regex":"([0-9]+)-(0[1-9]|1[012])-(0[1-9]|[12][0-9]|3[01])","transformers":null}],"rank":0,"redirect_code":302,"source":{"host":"","path":"/date/from/@marker","query":""},"target":"/date/target/@marker"}"#).expect("cannot deserialize");
    router.insert(route_1.into_route());

    router
}


#[test]
fn test_marker_type_date_1() {
    let router = setup_marker_type_date();
    let request = Request::new(r#"/date/from/2018-11-23"#.to_string(),None,None,
    None).to_http_request().expect("");
    let matched = router.match_request(&request);

    assert_eq!(!matched.is_empty(), true);

    let action = Action::from_routes_rule(matched, &request);

    assert_eq!(action.get_status_code(0), 302);
    let headers = action.filter_headers(Vec::new(), 0);
    assert_eq!(headers.len(), 1);

    let target_header = headers.first().unwrap();
    assert_eq!(target_header.name, "Location");
    assert_eq!(target_header.value, r#"/date/target/2018-11-23"#);
}

#[test]
fn test_marker_type_date_2() {
    let router = setup_marker_type_date();
    let request = Request::new(r#"/date/from/2018-23-11"#.to_string(),None,None,
    None).to_http_request().expect("");
    let matched = router.match_request(&request);

    assert_eq!(!matched.is_empty(), false);

}

#[test]
fn test_marker_type_date_3() {
    let router = setup_marker_type_date();
    let request = Request::new(r#"/date/from/some-13-01"#.to_string(),None,None,
    None).to_http_request().expect("");
    let matched = router.match_request(&request);

    assert_eq!(!matched.is_empty(), false);

}


fn setup_marker_type_datetime() -> Router<Rule> {
    let mut router = Router::<Rule>::default();

    let route_1: Rule = serde_json::from_str(r#"{"body_filters":null,"id":"datetime-rule","markers":[{"name":"marker","regex":"([0-9]+)-(0[1-9]|1[012])-(0[1-9]|[12][0-9]|3[01])T([01][0-9]|2[0-3]):([0-5][0-9]):([0-5][0-9]|60)(\\.[0-9]+)?(([Zz])|([\\+|\\-]([01][0-9]|2[0-3])(:?[03]0)?))","transformers":null}],"rank":0,"redirect_code":302,"source":{"host":"","path":"/datetime/from/@marker","query":""},"target":"/datetime/target/@marker"}"#).expect("cannot deserialize");
    router.insert(route_1.into_route());

    let route_2: Rule = serde_json::from_str(r#"{"body_filters":null,"id":"datetime-rule-with-transform","markers":[{"name":"marker","regex":"([0-9]+)-(0[1-9]|1[012])-(0[1-9]|[12][0-9]|3[01])T([01][0-9]|2[0-3]):([0-5][0-9]):([0-5][0-9]|60)(\\.[0-9]+)?(([Zz])|([\\+|\\-]([01][0-9]|2[0-3])(:?[03]0)?))","transformers":[{"options":{"from":"0","to":"10"},"type":"slice"}]}],"rank":0,"redirect_code":302,"source":{"host":"","path":"/datetime-transform/from/@marker","query":""},"target":"/datetime-transform/target/@marker"}"#).expect("cannot deserialize");
    router.insert(route_2.into_route());

    router
}


#[test]
fn test_marker_type_datetime_1() {
    let router = setup_marker_type_datetime();
    let request = Request::new(r#"/datetime/from/2018-07-15T14:59:12Z"#.to_string(),None,None,
    None).to_http_request().expect("");
    let matched = router.match_request(&request);

    assert_eq!(!matched.is_empty(), true);

    let action = Action::from_routes_rule(matched, &request);

    assert_eq!(action.get_status_code(0), 302);
    let headers = action.filter_headers(Vec::new(), 0);
    assert_eq!(headers.len(), 1);

    let target_header = headers.first().unwrap();
    assert_eq!(target_header.name, "Location");
    assert_eq!(target_header.value, r#"/datetime/target/2018-07-15T14:59:12Z"#);
}

#[test]
fn test_marker_type_datetime_2() {
    let router = setup_marker_type_datetime();
    let request = Request::new(r#"/datetime/from/2018-07-15T14:59:12+02:00"#.to_string(),None,None,
    None).to_http_request().expect("");
    let matched = router.match_request(&request);

    assert_eq!(!matched.is_empty(), true);

    let action = Action::from_routes_rule(matched, &request);

    assert_eq!(action.get_status_code(0), 302);
    let headers = action.filter_headers(Vec::new(), 0);
    assert_eq!(headers.len(), 1);

    let target_header = headers.first().unwrap();
    assert_eq!(target_header.name, "Location");
    assert_eq!(target_header.value, r#"/datetime/target/2018-07-15T14:59:12+02:00"#);
}

#[test]
fn test_marker_type_datetime_3() {
    let router = setup_marker_type_datetime();
    let request = Request::new(r#"/datetime/from/2018-07-15 14:59:12Z"#.to_string(),None,None,
    None).to_http_request().expect("");
    let matched = router.match_request(&request);

    assert_eq!(!matched.is_empty(), false);

}

#[test]
fn test_marker_type_datetime_4() {
    let router = setup_marker_type_datetime();
    let request = Request::new(r#"/datetime-transform/from/2018-07-15T14:59:12Z"#.to_string(),None,None,
    None).to_http_request().expect("");
    let matched = router.match_request(&request);

    assert_eq!(!matched.is_empty(), true);

    let action = Action::from_routes_rule(matched, &request);

    assert_eq!(action.get_status_code(0), 302);
    let headers = action.filter_headers(Vec::new(), 0);
    assert_eq!(headers.len(), 1);

    let target_header = headers.first().unwrap();
    assert_eq!(target_header.name, "Location");
    assert_eq!(target_header.value, r#"/datetime-transform/target/2018-07-15"#);
}


fn setup_marker_type_enum() -> Router<Rule> {
    let mut router = Router::<Rule>::default();

    let route_1: Rule = serde_json::from_str(r#"{"body_filters":null,"id":"enum-rule","markers":[{"name":"marker","regex":"(cat|dog|fish)","transformers":null}],"rank":0,"redirect_code":302,"source":{"host":"","path":"/enum/from/@marker","query":""},"target":"/enum/target/@marker"}"#).expect("cannot deserialize");
    router.insert(route_1.into_route());

    router
}


#[test]
fn test_marker_type_enum_1() {
    let router = setup_marker_type_enum();
    let request = Request::new(r#"/enum/from/cat"#.to_string(),None,None,
    None).to_http_request().expect("");
    let matched = router.match_request(&request);

    assert_eq!(!matched.is_empty(), true);

    let action = Action::from_routes_rule(matched, &request);

    assert_eq!(action.get_status_code(0), 302);
    let headers = action.filter_headers(Vec::new(), 0);
    assert_eq!(headers.len(), 1);

    let target_header = headers.first().unwrap();
    assert_eq!(target_header.name, "Location");
    assert_eq!(target_header.value, r#"/enum/target/cat"#);
}

#[test]
fn test_marker_type_enum_2() {
    let router = setup_marker_type_enum();
    let request = Request::new(r#"/enum/from/cats-eyes"#.to_string(),None,None,
    None).to_http_request().expect("");
    let matched = router.match_request(&request);

    assert_eq!(!matched.is_empty(), false);

}

#[test]
fn test_marker_type_enum_3() {
    let router = setup_marker_type_enum();
    let request = Request::new(r#"/enum/from/dog"#.to_string(),None,None,
    None).to_http_request().expect("");
    let matched = router.match_request(&request);

    assert_eq!(!matched.is_empty(), true);

    let action = Action::from_routes_rule(matched, &request);

    assert_eq!(action.get_status_code(0), 302);
    let headers = action.filter_headers(Vec::new(), 0);
    assert_eq!(headers.len(), 1);

    let target_header = headers.first().unwrap();
    assert_eq!(target_header.name, "Location");
    assert_eq!(target_header.value, r#"/enum/target/dog"#);
}

#[test]
fn test_marker_type_enum_4() {
    let router = setup_marker_type_enum();
    let request = Request::new(r#"/enum/from/dogville"#.to_string(),None,None,
    None).to_http_request().expect("");
    let matched = router.match_request(&request);

    assert_eq!(!matched.is_empty(), false);

}


fn setup_marker_type_integer() -> Router<Rule> {
    let mut router = Router::<Rule>::default();

    let route_1: Rule = serde_json::from_str(r#"{"body_filters":null,"id":"integer-max-rule","markers":[{"name":"marker","regex":"([0-9]|[1-3][0-9]|4[0-2])","transformers":null}],"rank":0,"redirect_code":302,"source":{"host":"","path":"/integer-max/from/@marker","query":""},"target":"/integer-max/target/@marker"}"#).expect("cannot deserialize");
    router.insert(route_1.into_route());

    let route_2: Rule = serde_json::from_str(r#"{"body_filters":null,"id":"integer-min-max-rule","markers":[{"name":"marker","regex":"(4[2-9]|[5-9][0-9]|[1-9][0-9]{2}|1[0-2][0-9]{2}|13[0-2][0-9]|133[0-7])","transformers":null}],"rank":0,"redirect_code":302,"source":{"host":"","path":"/integer-min-max/from/@marker","query":""},"target":"/integer-min-max/target/@marker"}"#).expect("cannot deserialize");
    router.insert(route_2.into_route());

    let route_3: Rule = serde_json::from_str(r#"{"body_filters":null,"id":"integer-min-rule","markers":[{"name":"marker","regex":"[1-3][0-9]{2,}|4([1-1][0-9]{1,}|[2-9][0-9]*)|[5-9][0-9]{1,}","transformers":null}],"rank":0,"redirect_code":302,"source":{"host":"","path":"/integer-min/from/@marker","query":""},"target":"/integer-min/target/@marker"}"#).expect("cannot deserialize");
    router.insert(route_3.into_route());

    let route_4: Rule = serde_json::from_str(r#"{"body_filters":null,"id":"integer-rule","markers":[{"name":"marker","regex":"[0-9]+","transformers":null}],"rank":0,"redirect_code":302,"source":{"host":"","path":"/integer/from/@marker","query":""},"target":"/integer/target/@marker"}"#).expect("cannot deserialize");
    router.insert(route_4.into_route());

    router
}


#[test]
fn test_marker_type_integer_1() {
    let router = setup_marker_type_integer();
    let request = Request::new(r#"/integer/from/2778"#.to_string(),None,None,
    None).to_http_request().expect("");
    let matched = router.match_request(&request);

    assert_eq!(!matched.is_empty(), true);

    let action = Action::from_routes_rule(matched, &request);

    assert_eq!(action.get_status_code(0), 302);
    let headers = action.filter_headers(Vec::new(), 0);
    assert_eq!(headers.len(), 1);

    let target_header = headers.first().unwrap();
    assert_eq!(target_header.name, "Location");
    assert_eq!(target_header.value, r#"/integer/target/2778"#);
}

#[test]
fn test_marker_type_integer_2() {
    let router = setup_marker_type_integer();
    let request = Request::new(r#"/integer/from/42l33t"#.to_string(),None,None,
    None).to_http_request().expect("");
    let matched = router.match_request(&request);

    assert_eq!(!matched.is_empty(), false);

}

#[test]
fn test_marker_type_integer_3() {
    let router = setup_marker_type_integer();
    let request = Request::new(r#"/integer/from/42-l33t"#.to_string(),None,None,
    None).to_http_request().expect("");
    let matched = router.match_request(&request);

    assert_eq!(!matched.is_empty(), false);

}

#[test]
fn test_marker_type_integer_4() {
    let router = setup_marker_type_integer();
    let request = Request::new(r#"/integer-min/from/112"#.to_string(),None,None,
    None).to_http_request().expect("");
    let matched = router.match_request(&request);

    assert_eq!(!matched.is_empty(), true);

    let action = Action::from_routes_rule(matched, &request);

    assert_eq!(action.get_status_code(0), 302);
    let headers = action.filter_headers(Vec::new(), 0);
    assert_eq!(headers.len(), 1);

    let target_header = headers.first().unwrap();
    assert_eq!(target_header.name, "Location");
    assert_eq!(target_header.value, r#"/integer-min/target/112"#);
}

#[test]
fn test_marker_type_integer_5() {
    let router = setup_marker_type_integer();
    let request = Request::new(r#"/integer-min/from/11"#.to_string(),None,None,
    None).to_http_request().expect("");
    let matched = router.match_request(&request);

    assert_eq!(!matched.is_empty(), false);

}

#[test]
fn test_marker_type_integer_6() {
    let router = setup_marker_type_integer();
    let request = Request::new(r#"/integer-max/from/11"#.to_string(),None,None,
    None).to_http_request().expect("");
    let matched = router.match_request(&request);

    assert_eq!(!matched.is_empty(), true);

    let action = Action::from_routes_rule(matched, &request);

    assert_eq!(action.get_status_code(0), 302);
    let headers = action.filter_headers(Vec::new(), 0);
    assert_eq!(headers.len(), 1);

    let target_header = headers.first().unwrap();
    assert_eq!(target_header.name, "Location");
    assert_eq!(target_header.value, r#"/integer-max/target/11"#);
}

#[test]
fn test_marker_type_integer_7() {
    let router = setup_marker_type_integer();
    let request = Request::new(r#"/integer-max/from/112"#.to_string(),None,None,
    None).to_http_request().expect("");
    let matched = router.match_request(&request);

    assert_eq!(!matched.is_empty(), false);

}

#[test]
fn test_marker_type_integer_8() {
    let router = setup_marker_type_integer();
    let request = Request::new(r#"/integer-min-max/from/806"#.to_string(),None,None,
    None).to_http_request().expect("");
    let matched = router.match_request(&request);

    assert_eq!(!matched.is_empty(), true);

    let action = Action::from_routes_rule(matched, &request);

    assert_eq!(action.get_status_code(0), 302);
    let headers = action.filter_headers(Vec::new(), 0);
    assert_eq!(headers.len(), 1);

    let target_header = headers.first().unwrap();
    assert_eq!(target_header.name, "Location");
    assert_eq!(target_header.value, r#"/integer-min-max/target/806"#);
}

#[test]
fn test_marker_type_integer_9() {
    let router = setup_marker_type_integer();
    let request = Request::new(r#"/integer-min-max/from/33"#.to_string(),None,None,
    None).to_http_request().expect("");
    let matched = router.match_request(&request);

    assert_eq!(!matched.is_empty(), false);

}

#[test]
fn test_marker_type_integer_10() {
    let router = setup_marker_type_integer();
    let request = Request::new(r#"/integer-min-max/from/2048"#.to_string(),None,None,
    None).to_http_request().expect("");
    let matched = router.match_request(&request);

    assert_eq!(!matched.is_empty(), false);

}


fn setup_marker_type_string() -> Router<Rule> {
    let mut router = Router::<Rule>::default();

    let route_1: Rule = serde_json::from_str(r#"{"body_filters":null,"id":"string-allowLowercaseAlphabet-specificCharacters-starting-containing-rule","markers":[{"name":"marker","regex":"JOHN\\-SNOW(([\\p{Ll}]|\\-)*?L33T([\\p{Ll}]|\\-)*?)+?","transformers":null}],"rank":0,"redirect_code":302,"source":{"host":"","path":"/string-allowLowercaseAlphabet-specificCharacters-starting-containing/from/@marker","query":""},"target":"/string-allowLowercaseAlphabet-specificCharacters-starting-containing/target/@marker"}"#).expect("cannot deserialize");
    router.insert(route_1.into_route());

    let route_2: Rule = serde_json::from_str(r#"{"body_filters":null,"id":"string-allowPercentEncodedChars-rule","markers":[{"name":"marker","regex":"(%[0-9A-Z]{2})+?","transformers":null}],"rank":0,"redirect_code":302,"source":{"host":"","path":"/string-allowPercentEncodedChars/from/@marker","query":""},"target":"/string-allowPercentEncodedChars/target/@marker"}"#).expect("cannot deserialize");
    router.insert(route_2.into_route());

    let route_3: Rule = serde_json::from_str(r#"{"body_filters":null,"id":"string-containing-rule","markers":[{"name":"marker","regex":"(L33T)+?","transformers":null}],"rank":0,"redirect_code":302,"source":{"host":"","path":"/string-containing/from/@marker","query":""},"target":"/string-containing/target/@marker"}"#).expect("cannot deserialize");
    router.insert(route_3.into_route());

    let route_4: Rule = serde_json::from_str(r#"{"body_filters":null,"id":"string-ending-rule","markers":[{"name":"marker","regex":"([\\p{Ll}]|\\-)+?JOHN\\-SNOW","transformers":null}],"rank":0,"redirect_code":302,"source":{"host":"","path":"/string-ending/from/@marker","query":""},"target":"/string-ending/target/@marker"}"#).expect("cannot deserialize");
    router.insert(route_4.into_route());

    let route_5: Rule = serde_json::from_str(r#"{"body_filters":null,"id":"string-lowercase-digits-allowPercentEncodedChars-rule","markers":[{"name":"marker","regex":"([\\p{Ll}0-9]|%[0-9A-Z]{2})+?","transformers":null}],"rank":0,"redirect_code":302,"source":{"host":"","path":"/string-lowercase-digits-allowPercentEncodedChars/from/@marker","query":""},"target":"/string-lowercase-digits-allowPercentEncodedChars/target/@marker"}"#).expect("cannot deserialize");
    router.insert(route_5.into_route());

    let route_6: Rule = serde_json::from_str(r#"{"body_filters":null,"id":"string-lowercase-rule","markers":[{"name":"marker","regex":"([\\p{Ll}])+?","transformers":null}],"rank":0,"redirect_code":302,"source":{"host":"","path":"/string-lowercase/from/@marker","query":""},"target":"/string-lowercase/target/@marker"}"#).expect("cannot deserialize");
    router.insert(route_6.into_route());

    let route_7: Rule = serde_json::from_str(r#"{"body_filters":null,"id":"string-lowercase-specificCharacters-emoji-rule","markers":[{"name":"marker","regex":"([\\p{Ll}]|\\-|🤘)+?","transformers":null}],"rank":0,"redirect_code":302,"source":{"host":"","path":"/string-lowercase-specificCharacters-emoji/from/@marker","query":""},"target":"/string-lowercase-specificCharacters-emoji/target/@marker"}"#).expect("cannot deserialize");
    router.insert(route_7.into_route());

    let route_8: Rule = serde_json::from_str(r#"{"body_filters":null,"id":"string-lowercase-uppercase-digits-allowPercentEncodedChars-specificCharacters-rule","markers":[{"name":"marker","regex":"([\\p{Ll}\\p{Lu}\\p{Lt}0-9]|\\-|\\.|\\(|\\)|%[0-9A-Z]{2})+?","transformers":null}],"rank":0,"redirect_code":302,"source":{"host":"","path":"/string-lowercase-uppercase-digits-allowPercentEncodedChars-specificCharacters/from/@marker","query":""},"target":"/string-lowercase-uppercase-digits-allowPercentEncodedChars-specificCharacters/target/@marker"}"#).expect("cannot deserialize");
    router.insert(route_8.into_route());

    let route_9: Rule = serde_json::from_str(r#"{"body_filters":null,"id":"string-lowercase-uppercase-digits-rule","markers":[{"name":"marker","regex":"([\\p{Ll}\\p{Lu}\\p{Lt}0-9])+?","transformers":null}],"rank":0,"redirect_code":302,"source":{"host":"","path":"/string-lowercase-uppercase-digits/from/@marker","query":""},"target":"/string-lowercase-uppercase-digits/target/@marker"}"#).expect("cannot deserialize");
    router.insert(route_9.into_route());

    let route_10: Rule = serde_json::from_str(r#"{"body_filters":null,"id":"string-rule","markers":[{"name":"marker","regex":"","transformers":null}],"rank":0,"redirect_code":302,"source":{"host":"","path":"/string/from/@marker","query":""},"target":"/string/target/@marker"}"#).expect("cannot deserialize");
    router.insert(route_10.into_route());

    let route_11: Rule = serde_json::from_str(r#"{"body_filters":null,"id":"string-specificCharacters-other-rule","markers":[{"name":"marker","regex":"(a|\\-|z)+?","transformers":null}],"rank":0,"redirect_code":302,"source":{"host":"","path":"/string-specificCharacters-other/from/@marker","query":""},"target":"/string-specificCharacters-other/target/@marker"}"#).expect("cannot deserialize");
    router.insert(route_11.into_route());

    let route_12: Rule = serde_json::from_str(r#"{"body_filters":null,"id":"string-specificCharacters-rule","markers":[{"name":"marker","regex":"(\\.|\\-|\\+|_|/)+?","transformers":null}],"rank":0,"redirect_code":302,"source":{"host":"","path":"/string-specificCharacters/from/@marker","query":""},"target":"/string-specificCharacters/target/@marker"}"#).expect("cannot deserialize");
    router.insert(route_12.into_route());

    let route_13: Rule = serde_json::from_str(r#"{"body_filters":null,"id":"string-starting-rule","markers":[{"name":"marker","regex":"JOHN\\-SNOW([\\p{Ll}]|\\-)+?","transformers":null}],"rank":0,"redirect_code":302,"source":{"host":"","path":"/string-starting/from/@marker","query":""},"target":"/string-starting/target/@marker"}"#).expect("cannot deserialize");
    router.insert(route_13.into_route());

    let route_14: Rule = serde_json::from_str(r#"{"body_filters":null,"id":"string-starting-shit-rule","markers":[{"name":"marker","regex":"\\(\\[A\\-Z\\]\\)\\+([\\p{Ll}]|\\-)+?","transformers":null}],"rank":0,"redirect_code":302,"source":{"host":"","path":"/string-starting-shit/from/@marker","query":""},"target":"/string-starting-shit/target/@marker"}"#).expect("cannot deserialize");
    router.insert(route_14.into_route());

    let route_15: Rule = serde_json::from_str(r#"{"body_filters":null,"id":"string-uppercase-rule","markers":[{"name":"marker","regex":"([\\p{Lu}\\p{Lt}])+?","transformers":null}],"rank":0,"redirect_code":302,"source":{"host":"","path":"/string-uppercase/from/@marker","query":""},"target":"/string-uppercase/target/@marker"}"#).expect("cannot deserialize");
    router.insert(route_15.into_route());

    router
}


#[test]
fn test_marker_type_string_1() {
    let router = setup_marker_type_string();
    let request = Request::new(r#"/string/from/coucou"#.to_string(),None,None,
    None).to_http_request().expect("");
    let matched = router.match_request(&request);

    assert_eq!(!matched.is_empty(), false);

}

#[test]
fn test_marker_type_string_2() {
    let router = setup_marker_type_string();
    let request = Request::new(r#"/string-lowercase/from/coucou"#.to_string(),None,None,
    None).to_http_request().expect("");
    let matched = router.match_request(&request);

    assert_eq!(!matched.is_empty(), true);

    let action = Action::from_routes_rule(matched, &request);

    assert_eq!(action.get_status_code(0), 302);
    let headers = action.filter_headers(Vec::new(), 0);
    assert_eq!(headers.len(), 1);

    let target_header = headers.first().unwrap();
    assert_eq!(target_header.name, "Location");
    assert_eq!(target_header.value, r#"/string-lowercase/target/coucou"#);
}

#[test]
fn test_marker_type_string_3() {
    let router = setup_marker_type_string();
    let request = Request::new(r#"/string-lowercase/from/COUCOU"#.to_string(),None,None,
    None).to_http_request().expect("");
    let matched = router.match_request(&request);

    assert_eq!(!matched.is_empty(), false);

}

#[test]
fn test_marker_type_string_4() {
    let router = setup_marker_type_string();
    let request = Request::new(r#"/string-lowercase/from/some-string"#.to_string(),None,None,
    None).to_http_request().expect("");
    let matched = router.match_request(&request);

    assert_eq!(!matched.is_empty(), false);

}

#[test]
fn test_marker_type_string_5() {
    let router = setup_marker_type_string();
    let request = Request::new(r#"/string-lowercase/from/l33t"#.to_string(),None,None,
    None).to_http_request().expect("");
    let matched = router.match_request(&request);

    assert_eq!(!matched.is_empty(), false);

}

#[test]
fn test_marker_type_string_6() {
    let router = setup_marker_type_string();
    let request = Request::new(r#"/string-uppercase/from/COUCOU"#.to_string(),None,None,
    None).to_http_request().expect("");
    let matched = router.match_request(&request);

    assert_eq!(!matched.is_empty(), true);

    let action = Action::from_routes_rule(matched, &request);

    assert_eq!(action.get_status_code(0), 302);
    let headers = action.filter_headers(Vec::new(), 0);
    assert_eq!(headers.len(), 1);

    let target_header = headers.first().unwrap();
    assert_eq!(target_header.name, "Location");
    assert_eq!(target_header.value, r#"/string-uppercase/target/COUCOU"#);
}

#[test]
fn test_marker_type_string_7() {
    let router = setup_marker_type_string();
    let request = Request::new(r#"/string-uppercase/from/coucou"#.to_string(),None,None,
    None).to_http_request().expect("");
    let matched = router.match_request(&request);

    assert_eq!(!matched.is_empty(), false);

}

#[test]
fn test_marker_type_string_8() {
    let router = setup_marker_type_string();
    let request = Request::new(r#"/string-uppercase/from/SOME-STRING"#.to_string(),None,None,
    None).to_http_request().expect("");
    let matched = router.match_request(&request);

    assert_eq!(!matched.is_empty(), false);

}

#[test]
fn test_marker_type_string_9() {
    let router = setup_marker_type_string();
    let request = Request::new(r#"/string-uppercase/from/L33T"#.to_string(),None,None,
    None).to_http_request().expect("");
    let matched = router.match_request(&request);

    assert_eq!(!matched.is_empty(), false);

}

#[test]
fn test_marker_type_string_10() {
    let router = setup_marker_type_string();
    let request = Request::new(r#"/string-lowercase-uppercase-digits/from/coucou"#.to_string(),None,None,
    None).to_http_request().expect("");
    let matched = router.match_request(&request);

    assert_eq!(!matched.is_empty(), true);

    let action = Action::from_routes_rule(matched, &request);

    assert_eq!(action.get_status_code(0), 302);
    let headers = action.filter_headers(Vec::new(), 0);
    assert_eq!(headers.len(), 1);

    let target_header = headers.first().unwrap();
    assert_eq!(target_header.name, "Location");
    assert_eq!(target_header.value, r#"/string-lowercase-uppercase-digits/target/coucou"#);
}

#[test]
fn test_marker_type_string_11() {
    let router = setup_marker_type_string();
    let request = Request::new(r#"/string-lowercase-uppercase-digits/from/COUCOU"#.to_string(),None,None,
    None).to_http_request().expect("");
    let matched = router.match_request(&request);

    assert_eq!(!matched.is_empty(), true);

    let action = Action::from_routes_rule(matched, &request);

    assert_eq!(action.get_status_code(0), 302);
    let headers = action.filter_headers(Vec::new(), 0);
    assert_eq!(headers.len(), 1);

    let target_header = headers.first().unwrap();
    assert_eq!(target_header.name, "Location");
    assert_eq!(target_header.value, r#"/string-lowercase-uppercase-digits/target/COUCOU"#);
}

#[test]
fn test_marker_type_string_12() {
    let router = setup_marker_type_string();
    let request = Request::new(r#"/string-lowercase-uppercase-digits/from/SOME-STRING"#.to_string(),None,None,
    None).to_http_request().expect("");
    let matched = router.match_request(&request);

    assert_eq!(!matched.is_empty(), false);

}

#[test]
fn test_marker_type_string_13() {
    let router = setup_marker_type_string();
    let request = Request::new(r#"/string-lowercase-uppercase-digits/from/l33t"#.to_string(),None,None,
    None).to_http_request().expect("");
    let matched = router.match_request(&request);

    assert_eq!(!matched.is_empty(), true);

    let action = Action::from_routes_rule(matched, &request);

    assert_eq!(action.get_status_code(0), 302);
    let headers = action.filter_headers(Vec::new(), 0);
    assert_eq!(headers.len(), 1);

    let target_header = headers.first().unwrap();
    assert_eq!(target_header.name, "Location");
    assert_eq!(target_header.value, r#"/string-lowercase-uppercase-digits/target/l33t"#);
}

#[test]
fn test_marker_type_string_14() {
    let router = setup_marker_type_string();
    let request = Request::new(r#"/string-lowercase-uppercase-digits/from/L33T"#.to_string(),None,None,
    None).to_http_request().expect("");
    let matched = router.match_request(&request);

    assert_eq!(!matched.is_empty(), true);

    let action = Action::from_routes_rule(matched, &request);

    assert_eq!(action.get_status_code(0), 302);
    let headers = action.filter_headers(Vec::new(), 0);
    assert_eq!(headers.len(), 1);

    let target_header = headers.first().unwrap();
    assert_eq!(target_header.name, "Location");
    assert_eq!(target_header.value, r#"/string-lowercase-uppercase-digits/target/L33T"#);
}

#[test]
fn test_marker_type_string_15() {
    let router = setup_marker_type_string();
    let request = Request::new(r#"/string-specificCharacters/from/-"#.to_string(),None,None,
    None).to_http_request().expect("");
    let matched = router.match_request(&request);

    assert_eq!(!matched.is_empty(), true);

    let action = Action::from_routes_rule(matched, &request);

    assert_eq!(action.get_status_code(0), 302);
    let headers = action.filter_headers(Vec::new(), 0);
    assert_eq!(headers.len(), 1);

    let target_header = headers.first().unwrap();
    assert_eq!(target_header.name, "Location");
    assert_eq!(target_header.value, r#"/string-specificCharacters/target/-"#);
}

#[test]
fn test_marker_type_string_16() {
    let router = setup_marker_type_string();
    let request = Request::new(r#"/string-specificCharacters/from/-_.+_-/._-_."#.to_string(),None,None,
    None).to_http_request().expect("");
    let matched = router.match_request(&request);

    assert_eq!(!matched.is_empty(), true);

    let action = Action::from_routes_rule(matched, &request);

    assert_eq!(action.get_status_code(0), 302);
    let headers = action.filter_headers(Vec::new(), 0);
    assert_eq!(headers.len(), 1);

    let target_header = headers.first().unwrap();
    assert_eq!(target_header.name, "Location");
    assert_eq!(target_header.value, r#"/string-specificCharacters/target/-_.+_-/._-_."#);
}

#[test]
fn test_marker_type_string_17() {
    let router = setup_marker_type_string();
    let request = Request::new(r#"/string-specificCharacters-other/from/z-a-z-a-zz"#.to_string(),None,None,
    None).to_http_request().expect("");
    let matched = router.match_request(&request);

    assert_eq!(!matched.is_empty(), true);

    let action = Action::from_routes_rule(matched, &request);

    assert_eq!(action.get_status_code(0), 302);
    let headers = action.filter_headers(Vec::new(), 0);
    assert_eq!(headers.len(), 1);

    let target_header = headers.first().unwrap();
    assert_eq!(target_header.name, "Location");
    assert_eq!(target_header.value, r#"/string-specificCharacters-other/target/z-a-z-a-zz"#);
}

#[test]
fn test_marker_type_string_18() {
    let router = setup_marker_type_string();
    let request = Request::new(r#"/string-specificCharacters-other/from/azerty"#.to_string(),None,None,
    None).to_http_request().expect("");
    let matched = router.match_request(&request);

    assert_eq!(!matched.is_empty(), false);

}

#[test]
fn test_marker_type_string_19() {
    let router = setup_marker_type_string();
    let request = Request::new(r#"/string-lowercase-specificCharacters-emoji/from/you-rock-dude-🤘"#.to_string(),None,None,
    None).to_http_request().expect("");
    let matched = router.match_request(&request);

    assert_eq!(!matched.is_empty(), true);

    let action = Action::from_routes_rule(matched, &request);

    assert_eq!(action.get_status_code(0), 302);
    let headers = action.filter_headers(Vec::new(), 0);
    assert_eq!(headers.len(), 1);

    let target_header = headers.first().unwrap();
    assert_eq!(target_header.name, "Location");
    assert_eq!(target_header.value, r#"/string-lowercase-specificCharacters-emoji/target/you-rock-dude-%F0%9F%A4%98"#);
}

#[test]
fn test_marker_type_string_20() {
    let router = setup_marker_type_string();
    let request = Request::new(r#"/string-starting/from/JOHN-SNOW-knows-nothing"#.to_string(),None,None,
    None).to_http_request().expect("");
    let matched = router.match_request(&request);

    assert_eq!(!matched.is_empty(), true);

    let action = Action::from_routes_rule(matched, &request);

    assert_eq!(action.get_status_code(0), 302);
    let headers = action.filter_headers(Vec::new(), 0);
    assert_eq!(headers.len(), 1);

    let target_header = headers.first().unwrap();
    assert_eq!(target_header.name, "Location");
    assert_eq!(target_header.value, r#"/string-starting/target/JOHN-SNOW-knows-nothing"#);
}

#[test]
fn test_marker_type_string_21() {
    let router = setup_marker_type_string();
    let request = Request::new(r#"/string-starting/from/you-know-nothing-JOHN-SNOW"#.to_string(),None,None,
    None).to_http_request().expect("");
    let matched = router.match_request(&request);

    assert_eq!(!matched.is_empty(), false);

}

#[test]
fn test_marker_type_string_22() {
    let router = setup_marker_type_string();
    let request = Request::new(r#"/string-starting-shit/from/COUCOU-you-know-nothing"#.to_string(),None,None,
    None).to_http_request().expect("");
    let matched = router.match_request(&request);

    assert_eq!(!matched.is_empty(), false);

}

#[test]
fn test_marker_type_string_23() {
    let router = setup_marker_type_string();
    let request = Request::new(r#"/string-starting-shit/from/([A-Z])+-knows-nothing"#.to_string(),None,None,
    None).to_http_request().expect("");
    let matched = router.match_request(&request);

    assert_eq!(!matched.is_empty(), true);

    let action = Action::from_routes_rule(matched, &request);

    assert_eq!(action.get_status_code(0), 302);
    let headers = action.filter_headers(Vec::new(), 0);
    assert_eq!(headers.len(), 1);

    let target_header = headers.first().unwrap();
    assert_eq!(target_header.name, "Location");
    assert_eq!(target_header.value, r#"/string-starting-shit/target/([A-Z])+-knows-nothing"#);
}

#[test]
fn test_marker_type_string_24() {
    let router = setup_marker_type_string();
    let request = Request::new(r#"/string-ending/from/JOHN-SNOW-knows-nothing"#.to_string(),None,None,
    None).to_http_request().expect("");
    let matched = router.match_request(&request);

    assert_eq!(!matched.is_empty(), false);

}

#[test]
fn test_marker_type_string_25() {
    let router = setup_marker_type_string();
    let request = Request::new(r#"/string-ending/from/you-know-nothing-JOHN-SNOW"#.to_string(),None,None,
    None).to_http_request().expect("");
    let matched = router.match_request(&request);

    assert_eq!(!matched.is_empty(), true);

    let action = Action::from_routes_rule(matched, &request);

    assert_eq!(action.get_status_code(0), 302);
    let headers = action.filter_headers(Vec::new(), 0);
    assert_eq!(headers.len(), 1);

    let target_header = headers.first().unwrap();
    assert_eq!(target_header.name, "Location");
    assert_eq!(target_header.value, r#"/string-ending/target/you-know-nothing-JOHN-SNOW"#);
}

#[test]
fn test_marker_type_string_26() {
    let router = setup_marker_type_string();
    let request = Request::new(r#"/string-ending/from/you-know-nothing-JOHN-SNOWR"#.to_string(),None,None,
    None).to_http_request().expect("");
    let matched = router.match_request(&request);

    assert_eq!(!matched.is_empty(), false);

}

#[test]
fn test_marker_type_string_27() {
    let router = setup_marker_type_string();
    let request = Request::new(r#"/string-allowPercentEncodedChars/from/%2B%3A%26"#.to_string(),None,None,
    None).to_http_request().expect("");
    let matched = router.match_request(&request);

    assert_eq!(!matched.is_empty(), true);

    let action = Action::from_routes_rule(matched, &request);

    assert_eq!(action.get_status_code(0), 302);
    let headers = action.filter_headers(Vec::new(), 0);
    assert_eq!(headers.len(), 1);

    let target_header = headers.first().unwrap();
    assert_eq!(target_header.name, "Location");
    assert_eq!(target_header.value, r#"/string-allowPercentEncodedChars/target/%2B%3A%26"#);
}

#[test]
fn test_marker_type_string_28() {
    let router = setup_marker_type_string();
    let request = Request::new(r#"/string-allowPercentEncodedChars/from/%3A"#.to_string(),None,None,
    None).to_http_request().expect("");
    let matched = router.match_request(&request);

    assert_eq!(!matched.is_empty(), true);

    let action = Action::from_routes_rule(matched, &request);

    assert_eq!(action.get_status_code(0), 302);
    let headers = action.filter_headers(Vec::new(), 0);
    assert_eq!(headers.len(), 1);

    let target_header = headers.first().unwrap();
    assert_eq!(target_header.name, "Location");
    assert_eq!(target_header.value, r#"/string-allowPercentEncodedChars/target/%3A"#);
}

#[test]
fn test_marker_type_string_29() {
    let router = setup_marker_type_string();
    let request = Request::new(r#"/string-allowPercentEncodedChars/from/%2B"#.to_string(),None,None,
    None).to_http_request().expect("");
    let matched = router.match_request(&request);

    assert_eq!(!matched.is_empty(), true);

    let action = Action::from_routes_rule(matched, &request);

    assert_eq!(action.get_status_code(0), 302);
    let headers = action.filter_headers(Vec::new(), 0);
    assert_eq!(headers.len(), 1);

    let target_header = headers.first().unwrap();
    assert_eq!(target_header.name, "Location");
    assert_eq!(target_header.value, r#"/string-allowPercentEncodedChars/target/%2B"#);
}

#[test]
fn test_marker_type_string_30() {
    let router = setup_marker_type_string();
    let request = Request::new(r#"/string-allowPercentEncodedChars/from/%26"#.to_string(),None,None,
    None).to_http_request().expect("");
    let matched = router.match_request(&request);

    assert_eq!(!matched.is_empty(), true);

    let action = Action::from_routes_rule(matched, &request);

    assert_eq!(action.get_status_code(0), 302);
    let headers = action.filter_headers(Vec::new(), 0);
    assert_eq!(headers.len(), 1);

    let target_header = headers.first().unwrap();
    assert_eq!(target_header.name, "Location");
    assert_eq!(target_header.value, r#"/string-allowPercentEncodedChars/target/%26"#);
}

#[test]
fn test_marker_type_string_31() {
    let router = setup_marker_type_string();
    let request = Request::new(r#"/string-allowPercentEncodedChars/from/0%2B0%3Dtoto"#.to_string(),None,None,
    None).to_http_request().expect("");
    let matched = router.match_request(&request);

    assert_eq!(!matched.is_empty(), false);

}

#[test]
fn test_marker_type_string_32() {
    let router = setup_marker_type_string();
    let request = Request::new(r#"/string-allowPercentEncodedChars/from/+:&"#.to_string(),None,None,
    None).to_http_request().expect("");
    let matched = router.match_request(&request);

    assert_eq!(!matched.is_empty(), false);

}

#[test]
fn test_marker_type_string_33() {
    let router = setup_marker_type_string();
    let request = Request::new(r#"/string-lowercase-digits-allowPercentEncodedChars/from/0%2B0%3Dtoto"#.to_string(),None,None,
    None).to_http_request().expect("");
    let matched = router.match_request(&request);

    assert_eq!(!matched.is_empty(), true);

    let action = Action::from_routes_rule(matched, &request);

    assert_eq!(action.get_status_code(0), 302);
    let headers = action.filter_headers(Vec::new(), 0);
    assert_eq!(headers.len(), 1);

    let target_header = headers.first().unwrap();
    assert_eq!(target_header.name, "Location");
    assert_eq!(target_header.value, r#"/string-lowercase-digits-allowPercentEncodedChars/target/0%2B0%3Dtoto"#);
}

#[test]
fn test_marker_type_string_34() {
    let router = setup_marker_type_string();
    let request = Request::new(r#"/string-lowercase-digits-allowPercentEncodedChars/from/0+0=toto"#.to_string(),None,None,
    None).to_http_request().expect("");
    let matched = router.match_request(&request);

    assert_eq!(!matched.is_empty(), false);

}

#[test]
fn test_marker_type_string_35() {
    let router = setup_marker_type_string();
    let request = Request::new(r#"/string-lowercase-uppercase-digits-allowPercentEncodedChars-specificCharacters/from/Medios-de-Comunicaci%C3%B3n-y-Creatividad"#.to_string(),None,None,
    None).to_http_request().expect("");
    let matched = router.match_request(&request);

    assert_eq!(!matched.is_empty(), true);

    let action = Action::from_routes_rule(matched, &request);

    assert_eq!(action.get_status_code(0), 302);
    let headers = action.filter_headers(Vec::new(), 0);
    assert_eq!(headers.len(), 1);

    let target_header = headers.first().unwrap();
    assert_eq!(target_header.name, "Location");
    assert_eq!(target_header.value, r#"/string-lowercase-uppercase-digits-allowPercentEncodedChars-specificCharacters/target/Medios-de-Comunicaci%C3%B3n-y-Creatividad"#);
}

#[test]
fn test_marker_type_string_36() {
    let router = setup_marker_type_string();
    let request = Request::new(r#"/string-lowercase-uppercase-digits-allowPercentEncodedChars-specificCharacters/from/Medios-de-Comunicación-y-Creatividad"#.to_string(),None,None,
    None).to_http_request().expect("");
    let matched = router.match_request(&request);

    assert_eq!(!matched.is_empty(), true);

    let action = Action::from_routes_rule(matched, &request);

    assert_eq!(action.get_status_code(0), 302);
    let headers = action.filter_headers(Vec::new(), 0);
    assert_eq!(headers.len(), 1);

    let target_header = headers.first().unwrap();
    assert_eq!(target_header.name, "Location");
    assert_eq!(target_header.value, r#"/string-lowercase-uppercase-digits-allowPercentEncodedChars-specificCharacters/target/Medios-de-Comunicaci%C3%B3n-y-Creatividad"#);
}

#[test]
fn test_marker_type_string_37() {
    let router = setup_marker_type_string();
    let request = Request::new(r#"/string-containing/from/L33T"#.to_string(),None,None,
    None).to_http_request().expect("");
    let matched = router.match_request(&request);

    assert_eq!(!matched.is_empty(), true);

    let action = Action::from_routes_rule(matched, &request);

    assert_eq!(action.get_status_code(0), 302);
    let headers = action.filter_headers(Vec::new(), 0);
    assert_eq!(headers.len(), 1);

    let target_header = headers.first().unwrap();
    assert_eq!(target_header.name, "Location");
    assert_eq!(target_header.value, r#"/string-containing/target/L33T"#);
}

#[test]
fn test_marker_type_string_38() {
    let router = setup_marker_type_string();
    let request = Request::new(r#"/string-containing/from/L33TL33T"#.to_string(),None,None,
    None).to_http_request().expect("");
    let matched = router.match_request(&request);

    assert_eq!(!matched.is_empty(), true);

    let action = Action::from_routes_rule(matched, &request);

    assert_eq!(action.get_status_code(0), 302);
    let headers = action.filter_headers(Vec::new(), 0);
    assert_eq!(headers.len(), 1);

    let target_header = headers.first().unwrap();
    assert_eq!(target_header.name, "Location");
    assert_eq!(target_header.value, r#"/string-containing/target/L33TL33T"#);
}

#[test]
fn test_marker_type_string_39() {
    let router = setup_marker_type_string();
    let request = Request::new(r#"/string-containing/from/42-L33T-42"#.to_string(),None,None,
    None).to_http_request().expect("");
    let matched = router.match_request(&request);

    assert_eq!(!matched.is_empty(), false);

}

#[test]
fn test_marker_type_string_40() {
    let router = setup_marker_type_string();
    let request = Request::new(r#"/string-allowLowercaseAlphabet-specificCharacters-starting-containing/from/JOHN-SNOW-L33T-knows-nothing"#.to_string(),None,None,
    None).to_http_request().expect("");
    let matched = router.match_request(&request);

    assert_eq!(!matched.is_empty(), true);

    let action = Action::from_routes_rule(matched, &request);

    assert_eq!(action.get_status_code(0), 302);
    let headers = action.filter_headers(Vec::new(), 0);
    assert_eq!(headers.len(), 1);

    let target_header = headers.first().unwrap();
    assert_eq!(target_header.name, "Location");
    assert_eq!(target_header.value, r#"/string-allowLowercaseAlphabet-specificCharacters-starting-containing/target/JOHN-SNOW-L33T-knows-nothing"#);
}

#[test]
fn test_marker_type_string_41() {
    let router = setup_marker_type_string();
    let request = Request::new(r#"/string-allowLowercaseAlphabet-specificCharacters-starting-containing/from/JOHN-SNOWL33T"#.to_string(),None,None,
    None).to_http_request().expect("");
    let matched = router.match_request(&request);

    assert_eq!(!matched.is_empty(), true);

    let action = Action::from_routes_rule(matched, &request);

    assert_eq!(action.get_status_code(0), 302);
    let headers = action.filter_headers(Vec::new(), 0);
    assert_eq!(headers.len(), 1);

    let target_header = headers.first().unwrap();
    assert_eq!(target_header.name, "Location");
    assert_eq!(target_header.value, r#"/string-allowLowercaseAlphabet-specificCharacters-starting-containing/target/JOHN-SNOWL33T"#);
}

#[test]
fn test_marker_type_string_42() {
    let router = setup_marker_type_string();
    let request = Request::new(r#"/string-allowLowercaseAlphabet-specificCharacters-starting-containing/from/L33T-JOHN-SNOW-knows-nothing"#.to_string(),None,None,
    None).to_http_request().expect("");
    let matched = router.match_request(&request);

    assert_eq!(!matched.is_empty(), false);

}

#[test]
fn test_marker_type_string_43() {
    let router = setup_marker_type_string();
    let request = Request::new(r#"/string-allowLowercaseAlphabet-specificCharacters-starting-containing/from/JOHN-SNOW-l33t"#.to_string(),None,None,
    None).to_http_request().expect("");
    let matched = router.match_request(&request);

    assert_eq!(!matched.is_empty(), false);

}

#[test]
fn test_marker_type_string_44() {
    let router = setup_marker_type_string();
    let request = Request::new(r#"/string-allowLowercaseAlphabet-specificCharacters-starting-containing/from/JOHN-SNOW-L3a3t"#.to_string(),None,None,
    None).to_http_request().expect("");
    let matched = router.match_request(&request);

    assert_eq!(!matched.is_empty(), false);

}


fn setup_marker_type_uuid() -> Router<Rule> {
    let mut router = Router::<Rule>::default();

    let route_1: Rule = serde_json::from_str(r#"{"body_filters":null,"id":"uuid-rule","markers":[{"name":"marker","regex":"[a-fA-F0-9]{8}-[a-fA-F0-9]{4}-[a-fA-F0-9]{4}-[a-fA-F0-9]{4}-[a-fA-F0-9]{12}","transformers":null}],"rank":0,"redirect_code":302,"source":{"host":"","path":"/uuid/from/@marker","query":""},"target":"/uuid/target/@marker"}"#).expect("cannot deserialize");
    router.insert(route_1.into_route());

    router
}


#[test]
fn test_marker_type_uuid_1() {
    let router = setup_marker_type_uuid();
    let request = Request::new(r#"/uuid/from/f6883ff9-f163-43d7-8177-bfa24277fd20"#.to_string(),None,None,
    None).to_http_request().expect("");
    let matched = router.match_request(&request);

    assert_eq!(!matched.is_empty(), true);

    let action = Action::from_routes_rule(matched, &request);

    assert_eq!(action.get_status_code(0), 302);
    let headers = action.filter_headers(Vec::new(), 0);
    assert_eq!(headers.len(), 1);

    let target_header = headers.first().unwrap();
    assert_eq!(target_header.name, "Location");
    assert_eq!(target_header.value, r#"/uuid/target/f6883ff9-f163-43d7-8177-bfa24277fd20"#);
}

#[test]
fn test_marker_type_uuid_2() {
    let router = setup_marker_type_uuid();
    let request = Request::new(r#"/uuid/from/HELLO"#.to_string(),None,None,
    None).to_http_request().expect("");
    let matched = router.match_request(&request);

    assert_eq!(!matched.is_empty(), false);

}

#[test]
fn test_marker_type_uuid_3() {
    let router = setup_marker_type_uuid();
    let request = Request::new(r#"/uuid/from/f688-3ff9-f16343d78177bfa2-4277-fd20"#.to_string(),None,None,
    None).to_http_request().expect("");
    let matched = router.match_request(&request);

    assert_eq!(!matched.is_empty(), false);

}


fn setup_rule_query_with_plus() -> Router<Rule> {
    let mut router = Router::<Rule>::default();

    let route_1: Rule = serde_json::from_str(r#"{"body_filters":null,"id":"host-path-query-double-quotes","markers":null,"rank":0,"redirect_code":301,"source":{"host":"example.org","path":"/query-plus","query":"foo=bar+baz"},"target":"/target"}"#).expect("cannot deserialize");
    router.insert(route_1.into_route());

    router
}


#[test]
fn test_rule_query_with_plus_1() {
    let router = setup_rule_query_with_plus();
    let request = Request::new(r#"/query-plus?foo=bar+baz"#.to_string(),Some(r#"example.org"#.to_string()),Some(r#"http"#.to_string()),
    None).to_http_request().expect("");
    let matched = router.match_request(&request);

    assert_eq!(!matched.is_empty(), true);

    let action = Action::from_routes_rule(matched, &request);

    assert_eq!(action.get_status_code(0), 301);
    let headers = action.filter_headers(Vec::new(), 0);
    assert_eq!(headers.len(), 1);

    let target_header = headers.first().unwrap();
    assert_eq!(target_header.name, "Location");
    assert_eq!(target_header.value, r#"/target"#);
}

#[test]
fn test_rule_query_with_plus_2() {
    let router = setup_rule_query_with_plus();
    let request = Request::new(r#"/query-plus?foo=bar baz"#.to_string(),Some(r#"example.org"#.to_string()),Some(r#"http"#.to_string()),
    None).to_http_request().expect("");
    let matched = router.match_request(&request);

    assert_eq!(!matched.is_empty(), true);

    let action = Action::from_routes_rule(matched, &request);

    assert_eq!(action.get_status_code(0), 301);
    let headers = action.filter_headers(Vec::new(), 0);
    assert_eq!(headers.len(), 1);

    let target_header = headers.first().unwrap();
    assert_eq!(target_header.name, "Location");
    assert_eq!(target_header.value, r#"/target"#);
}

#[test]
fn test_rule_query_with_plus_3() {
    let router = setup_rule_query_with_plus();
    let request = Request::new(r#"/query-plus?foo=bar%20baz"#.to_string(),Some(r#"example.org"#.to_string()),Some(r#"http"#.to_string()),
    None).to_http_request().expect("");
    let matched = router.match_request(&request);

    assert_eq!(!matched.is_empty(), true);

    let action = Action::from_routes_rule(matched, &request);

    assert_eq!(action.get_status_code(0), 301);
    let headers = action.filter_headers(Vec::new(), 0);
    assert_eq!(headers.len(), 1);

    let target_header = headers.first().unwrap();
    assert_eq!(target_header.name, "Location");
    assert_eq!(target_header.value, r#"/target"#);
}


fn setup_rule_querystring() -> Router<Rule> {
    let mut router = Router::<Rule>::default();

    let route_1: Rule = serde_json::from_str(r#"{"body_filters":null,"id":"host-path-query-double-quotes","markers":null,"rank":0,"redirect_code":301,"source":{"host":"example.org","path":"/host-path-query","query":"foo&bar=yolo"},"target":"/target"}"#).expect("cannot deserialize");
    router.insert(route_1.into_route());

    router
}


#[test]
fn test_rule_querystring_1() {
    let router = setup_rule_querystring();
    let request = Request::new(r#"/host-path-query?foo&bar=yolo"#.to_string(),Some(r#"example.org"#.to_string()),Some(r#"http"#.to_string()),
    None).to_http_request().expect("");
    let matched = router.match_request(&request);

    assert_eq!(!matched.is_empty(), true);

    let action = Action::from_routes_rule(matched, &request);

    assert_eq!(action.get_status_code(0), 301);
    let headers = action.filter_headers(Vec::new(), 0);
    assert_eq!(headers.len(), 1);

    let target_header = headers.first().unwrap();
    assert_eq!(target_header.name, "Location");
    assert_eq!(target_header.value, r#"/target"#);
}

#[test]
fn test_rule_querystring_2() {
    let router = setup_rule_querystring();
    let request = Request::new(r#"/host-path-query?foo=&bar=yolo"#.to_string(),Some(r#"example.org"#.to_string()),Some(r#"http"#.to_string()),
    None).to_http_request().expect("");
    let matched = router.match_request(&request);

    assert_eq!(!matched.is_empty(), true);

    let action = Action::from_routes_rule(matched, &request);

    assert_eq!(action.get_status_code(0), 301);
    let headers = action.filter_headers(Vec::new(), 0);
    assert_eq!(headers.len(), 1);

    let target_header = headers.first().unwrap();
    assert_eq!(target_header.name, "Location");
    assert_eq!(target_header.value, r#"/target"#);
}


fn setup_rule_with_quotes() -> Router<Rule> {
    let mut router = Router::<Rule>::default();

    let route_1: Rule = serde_json::from_str(r#"{"body_filters":null,"id":"host-path-query-double-quotes","markers":null,"rank":0,"redirect_code":301,"source":{"host":"example.org","path":"/host-path-query-double-quotes","query":"gender.nl-NL=Dames%22,%22Heren%22,%22Kinderens"},"target":"/target?gender=Dames&gender=Heren&gender=Kinderen"}"#).expect("cannot deserialize");
    router.insert(route_1.into_route());

    router
}


#[test]
fn test_rule_with_quotes_1() {
    let router = setup_rule_with_quotes();
    let request = Request::new(r#"/host-path-query-double-quotes?gender.nl-NL=Dames%22,%22Heren%22,%22Kinderens"#.to_string(),Some(r#"example.org"#.to_string()),Some(r#"http"#.to_string()),
    None).to_http_request().expect("");
    let matched = router.match_request(&request);

    assert_eq!(!matched.is_empty(), true);

    let action = Action::from_routes_rule(matched, &request);

    assert_eq!(action.get_status_code(0), 301);
    let headers = action.filter_headers(Vec::new(), 0);
    assert_eq!(headers.len(), 1);

    let target_header = headers.first().unwrap();
    assert_eq!(target_header.name, "Location");
    assert_eq!(target_header.value, r#"/target?gender=Dames&gender=Heren&gender=Kinderen"#);
}


fn setup_rule_with_slash() -> Router<Rule> {
    let mut router = Router::<Rule>::default();

    let route_1: Rule = serde_json::from_str(r#"{"body_filters":null,"id":"rule-with-slash","markers":null,"rank":0,"redirect_code":302,"source":{"host":"","path":"/foo/","query":""},"target":"/bar/"}"#).expect("cannot deserialize");
    router.insert(route_1.into_route());

    let route_2: Rule = serde_json::from_str(r#"{"body_filters":null,"id":"rule-without-slash","markers":null,"rank":0,"redirect_code":302,"source":{"host":"","path":"/foo","query":""},"target":"/bar"}"#).expect("cannot deserialize");
    router.insert(route_2.into_route());

    router
}


#[test]
fn test_rule_with_slash_1() {
    let router = setup_rule_with_slash();
    let request = Request::new(r#"/foo"#.to_string(),None,None,
    None).to_http_request().expect("");
    let matched = router.match_request(&request);

    assert_eq!(!matched.is_empty(), true);

    let action = Action::from_routes_rule(matched, &request);

    assert_eq!(action.get_status_code(0), 302);
    let headers = action.filter_headers(Vec::new(), 0);
    assert_eq!(headers.len(), 1);

    let target_header = headers.first().unwrap();
    assert_eq!(target_header.name, "Location");
    assert_eq!(target_header.value, r#"/bar"#);
}

#[test]
fn test_rule_with_slash_2() {
    let router = setup_rule_with_slash();
    let request = Request::new(r#"/foo/"#.to_string(),None,None,
    None).to_http_request().expect("");
    let matched = router.match_request(&request);

    assert_eq!(!matched.is_empty(), true);

    let action = Action::from_routes_rule(matched, &request);

    assert_eq!(action.get_status_code(0), 302);
    let headers = action.filter_headers(Vec::new(), 0);
    assert_eq!(headers.len(), 1);

    let target_header = headers.first().unwrap();
    assert_eq!(target_header.name, "Location");
    assert_eq!(target_header.value, r#"/bar/"#);
}


}
