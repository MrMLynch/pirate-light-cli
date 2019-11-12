pub fn get_closest_checkpoint(chain_name: &str, height: u64) ->  Option<(u64, &'static str, &'static str)> {
    match chain_name {
        "test" => get_test_checkpoint(height),
        "main" => get_main_checkpoint(height),
        _      => None
    }
}

fn get_test_checkpoint(height: u64) ->  Option<(u64, &'static str, &'static str)> {
    let checkpoints: Vec<(u64, &str, &str)> = vec![
        (560000, "000000004ea18707105a30716940b95420ace34d0dd71caddf47753155b965e7",
                 ""
        ),
        (595000, "00000000c49d7db4ebb7ec6ffdc145efc18b00c7fe21e9e06d2ebaef5d60c3f7",
                 ""
        )
    ];

    find_checkpoint(height, checkpoints)
}


fn get_main_checkpoint(height: u64) ->  Option<(u64, &'static str, &'static str)> {
    let checkpoints: Vec<(u64, &str, &str)> = vec![
        (610000, "000000000783d5b69b946f025e5566fa7df913d19963e05c15b0214a6398d986",
                 "0108a22d91b86cdd3451ad4be7e56a4735fdd0723622ddb75308f8b7533924a30201dfacac37748f7a9497a11144ada25b3ae6d28e3920ce6d638690ddcec25e984e1301a70396566fd687382bea6b868db055e448d434cc9631a3b0f7d98caf03ecbf2200000001227767fe1d2551d4a4850c3f45e5d328d20ada973dfc88c43858375a11770827012c70535d52d8da6103e2caac2eed0e15ccf86eff558c486447d2b595376e7c58000151143af46ce08eba1acb06a4dc7cf918db2ff361a5d3191b00a3b74e03f2533f01aadbc8525a8198ddbf36517b25f68c267a168c32239657f4d19107fd647e850601c6a42fd2bd84c48ada0d3155caec9f9a09ea3702d151508814a89ca64d5f1d6d000001109826e860ab2d5eee6af98954f88993ebc77c339fb10ce00fa1f022f12fd921013d239d12a353ea6a8e72720c395c448928a203e7d4ac18a9befd45a8bdb35e6c0105a213b81e22f1c69bbec210c6e3670e9f89ebb12762b8073a4645fced3ed6480160eed5dec89f889e7320fb965c3239c81b941ed2dc6edeac426be9a30f2e3b06000114067a827bbc4717638ba7699b53b1a85f6c57f2718d00718d35ec2fc82b672001634120b271dc798dc6438b8aac6bbd122b784b7229455885db6815c327dbb71d"
        )
    ];

    find_checkpoint(height, checkpoints)
}

fn find_checkpoint(height: u64, chkpts: Vec<(u64, &'static str, &'static str)>) -> Option<(u64, &'static str, &'static str)> {
    // Find the closest checkpoint
    let mut heights = chkpts.iter().map(|(h, _, _)| *h as u64).collect::<Vec<_>>();
    heights.sort();

    match get_first_lower_than(height, heights) {
        Some(closest_height) => {
            chkpts.iter().find(|(h, _, _)| *h ==  closest_height).map(|t| *t)
        },
        None    => None
    }
}

fn get_first_lower_than(height: u64, heights: Vec<u64>) -> Option<u64> {
    // If it's before the first checkpoint, return None. 
    if heights.len() == 0 || height < heights[0] {
        return None;
    }

    for (i, h) in heights.iter().enumerate() {
        if height < *h {
            return Some(heights[i-1]);
        }
    }

    return Some(*heights.last().unwrap());
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn test_lower_than() {
        assert_eq!(get_first_lower_than( 9, vec![10, 30, 40]), None);
        assert_eq!(get_first_lower_than(10, vec![10, 30, 40]).unwrap(), 10);
        assert_eq!(get_first_lower_than(11, vec![10, 30, 40]).unwrap(), 10);
        assert_eq!(get_first_lower_than(29, vec![10, 30, 40]).unwrap(), 10);
        assert_eq!(get_first_lower_than(30, vec![10, 30, 40]).unwrap(), 30);
        assert_eq!(get_first_lower_than(40, vec![10, 30, 40]).unwrap(), 40);
        assert_eq!(get_first_lower_than(41, vec![10, 30, 40]).unwrap(), 40);
        assert_eq!(get_first_lower_than(99, vec![10, 30, 40]).unwrap(), 40);
    }

    #[test]
    fn test_checkpoints() {
        assert_eq!(get_test_checkpoint(500000), None);
        assert_eq!(get_test_checkpoint(560000).unwrap().0, 560000);
        assert_eq!(get_test_checkpoint(585000).unwrap().0, 560000);
        assert_eq!(get_test_checkpoint(5950000).unwrap().0, 595000);
        assert_eq!(get_test_checkpoint(597000).unwrap().0, 595000);

        assert_eq!(get_main_checkpoint(500000), None);
        assert_eq!(get_main_checkpoint(610000).unwrap().0, 610000);
        assert_eq!(get_main_checkpoint(617000).unwrap().0, 610000);
    }

}
