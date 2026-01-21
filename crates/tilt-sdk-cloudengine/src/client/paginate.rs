use crate::error::{ComputeError, Result};
use std::future::Future;

const DEFAULT_PER_PAGE: u32 = 100;
const MAX_PAGES: u32 = 100;

pub async fn paginate<T, F, Fut>(
    limit: Option<u32>,
    page: Option<u32>,
    service: crate::error::Service,
    endpoint: &str,
    mut fetch_page: F,
) -> Result<Vec<T>>
where
    F: FnMut(u32, u32) -> Fut,
    Fut: Future<Output = Result<(Vec<T>, u32)>>,
{
    match limit {
        Some(input_limit) => {
            let limit = input_limit.min(MAX_PAGES);
            let page = page.unwrap_or(1);
            let (items, _) = fetch_page(page, limit).await?;
            Ok(items)
        }
        None => {
            let mut all_items: Vec<T> = Vec::new();
            let mut current_page = 1;
            let per_page = DEFAULT_PER_PAGE;

            loop {
                let (items, total_count) = fetch_page(current_page, per_page).await?;
                all_items.extend(items);

                if all_items.len() >= total_count as usize {
                    break;
                }

                current_page += 1;

                if current_page > MAX_PAGES {
                    return Err(ComputeError::validation(
                        service,
                        Some(endpoint),
                        format!(
                            "Pagination limit reached. Fetched {} of {} records",
                            all_items.len(),
                            total_count
                        ),
                    ));
                }
            }

            Ok(all_items)
        }
    }
}
