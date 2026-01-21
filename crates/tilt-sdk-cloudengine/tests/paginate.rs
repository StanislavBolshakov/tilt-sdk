//! Tests for pagination module

use tilt_sdk_cloudengine::client::paginate::paginate;
use tilt_sdk_cloudengine::error::Service;

#[tokio::test]
async fn test_pagination_with_limit() {
    let items: Vec<u32> = vec![1, 2, 3];

    let result = paginate(
        Some(10),
        Some(1),
        Service::ComputeApi,
        "/test",
        |_page, _limit| {
            let items = items.clone();
            async move { Ok((items, 3)) }
        },
    )
    .await
    .unwrap();

    assert_eq!(result, vec![1, 2, 3]);
}

#[tokio::test]
async fn test_pagination_without_limit_fetches_all() {
    let page1 = vec![1, 2, 3];
    let page2 = vec![4, 5, 6];
    let page1_clone = page1.clone();
    let page2_clone = page2.clone();

    let _fetch_count = 0;
    let result = paginate(
        None,
        None,
        Service::ComputeApi,
        "/test",
        move |page, _limit| {
            let page1 = page1_clone.clone();
            let page2 = page2_clone.clone();
            async move {
                if page == 1 {
                    Ok((page1, 6))
                } else {
                    Ok((page2, 6))
                }
            }
        },
    )
    .await
    .unwrap();

    assert_eq!(result, vec![1, 2, 3, 4, 5, 6]);
}
