use yuri::Uri;
use yuri::{Fragment, Query, Scheme};

fn main() {
    let s = "https://foo:secret@foobar.test:666/?q=a&m=s#fragemnt";

    let uri = Uri::new(s).expect("Failed to parse URI");

    assert_eq!(uri.scheme, Scheme::Http(true));

    if let Some(ref authority) = uri.authority {
        assert_eq!(authority.raw_host, "foobar.test");
        assert_eq!(authority.port, Some(666));

        if let Some(userinfo) = &(authority).userinfo {
            assert_eq!(userinfo.raw_user, "foo");
            assert_eq!(userinfo.raw_authorization(), Some("secret"));
        }
    }

    assert_eq!(
        uri.query,
        Some(Query {
            raw_query: "q=a&m=s"
        })
    );
    assert_eq!(
        uri.fragment,
        Some(Fragment {
            raw_fragment: "fragemnt"
        })
    );

    println!("{} -> {:?}", s, uri);
}
