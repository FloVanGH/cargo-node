use super::*;

#[test]
fn test_into() {
    let visibility: Visibility = "Hidden".into();
    assert_eq!(visibility.0, VisibilityValue::Hidden);

    let visibility: Visibility = "hidden".into();
    assert_eq!(visibility.0, VisibilityValue::Hidden);

    let visibility: Visibility = "Collapsed".into();
    assert_eq!(visibility.0, VisibilityValue::Collapsed);

    let visibility: Visibility = "collapsed".into();
    assert_eq!(visibility.0, VisibilityValue::Collapsed);

    let visibility: Visibility = "Visible".into();
    assert_eq!(visibility.0, VisibilityValue::Visible);

    let visibility: Visibility = "visible".into();
    assert_eq!(visibility.0, VisibilityValue::Visible);

    let visibility: Visibility = "other".into();
    assert_eq!(visibility.0, VisibilityValue::Visible);
}
