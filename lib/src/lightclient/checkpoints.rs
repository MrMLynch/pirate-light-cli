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
                 "004ba0224fcac03c68cd503ef3470c2908675b8b5d0741956d60912feb80a920f410911ca237557ffcbe0d99c3a98e2963f152a4624cc0e0853ce5000c5d7914413df87948a9793f728467bbb2f4ff25f97b84310710f1d1a90f6d19cd8881d6c19479c8a09c2a54cf198282e0480dd7ee7ee8a83fdd69140ee244d9e9971ce3643f70741233d887a4f328ae86c972010f5eb75d492ee224206ba5ecbc473f9fcf45923fb4b99b2c00c0b524c5092f8d4a78403b5486a9c856c8fb0bcf3825f787cc196136fc7896df84f22856e0c21cbc4a068a0304ecd580d1899130e02f37b7a851ea79af221bef668a654ab8d76aef9468ff3ad9f1339d14f8031d32b119d671322be630ac2ef0fe8163143a5ac34857767cb003e476c545403a3aa6f30f329e0c961635210621a062ee2d398e04345b81c330a93d84aa574534ba71df0ad0ce738c73c41a4042bc8972d1d29c8a00bf64a68eb5e0c5cf2b86e12f7428cb129099ba4304bed75c58049ba178e9610c4c3391d91d733844f72f5abdfd7b980622fff4a7ea165716d6c905beed5e5703243a626d08a7b4ac1cb9b7f53f1b6d0edeef0903043bc1c283102bf2ed9372513b324a7aab5e0e76033fbe8f5e8b4436cbc170f7d04462709fbd752b911152663bafa299bb9a751459eeb8acf21de4f20ea428d684a9691ba54afeb82584c33941ede0ff9912d4030c5cfae3f5dff7b1ac274f9c64d9eb2db5dd5ba830760d30514c4e7e9855a4e3ca6568fe370c1559e21a79b39a5a8ae69188bb11cf0fbb88420273d412a25de473156964cd9161ed88f87be7a276cc7d3a14a4158c711f0518325793a913b073fd31575cfe5fcd1335395f0855a2d71735248443dd2cdc414388d7521318d0da6aa666311ff3dbc21340ba560137c4970b4b1f235b1352197e44e487da4536f8006ee048f7faf8004d2bce20c3e263c5fae1f4b0acbf2f364edbb9f81037157b711a2e2efedcc87f726965f6e9a5bf7a100159b22c485b08c10593032f3554038d35d355d9cb1c50719bb6781a79e7e6a28c30c99a9ee8173f318c04041ba01aaca8a58944c0a1eb4e6f0536c7ce9ea511edb34fd78f8153a784f178b876e58e2f77fa24e82533d96d31d746850b5e97dbeef3479620711e3d3330f5a7f17411bc51e132c39e7e53a5e8f30f57097201ebfe172ee21429b9ea3929bede56334098fae6212ffa5dc9a15fecc75f05383e2d549aa3908d3e1f402461eb6dec2845839bd0a82d2648999297385fe88d3d412690f892996b0920f92ae5f24a3af52d198f8403e47b445383d32cae7be46550476c66c0c9bbe4db166ad5ce486de22fc7ff1331c2e674b0f7398cb07412fd0f202c0f880db3bb726737a620ecfae5fa92f82b46be76ab14d7f72b25049928eb3762ec09f866f000a79a66bf02406cefb46174ffbba31d554a15251f5b0d24dbcdd7b8eeeadcf6aea14b13ae1378db2bfa259c527417e51f1788b414827044efaad78b9ac6e52e7f3f37d551940f0a4a8728437ed6420acd9dd56f0421b1eb5456801d38b032761b9ae3e8b1b7f87b9a156ee6d640cd0594760a21e4b27b3331f04bb06e7f049915018e117e93f027764fffd49635ba747cb31528f284f24111014fe211650c907903b98472de8fc803e5aed9274d792b7235d736c4fe710acfcd9d8761196c0588985f0d93248872942c799b64b463c5c36906585b6bd0e66e8b658f83c4e26c851d7a6dfa00cf163ea4cd446682a58baf13658d605e2cf2c46a821915a04efd8fde3a475ddd17fd9c78af722807926ba931668dc1f024b251d3a7076315425bb6667737cdf21e3fb7fa84d27511a71a7316f8f514216ed63920ff32c3778f1911203b9dbe5a576275106ef15259d7d1"
        ),
        (595000, "00000000c49d7db4ebb7ec6ffdc145efc18b00c7fe21e9e06d2ebaef5d60c3f7",
                 "002a0950c23bafebf9fe50c2a7b9c1cce2af89225e14097261405666119547c19dd3430d5c89951908f9018485bbe21610f4fa1941890d18772d7d4e188bf4070f2078e90be6fb4244a0d75dbdb7e076a187d7060c50e2622189b4413468b1256b76a0f5f887fdb3c73a0ec24f01e758797269e65890f6ea9ec03bdb3fa32a77d708795a3f9b51dbb8ce70cb63db6628dbd92730cf4b1f5d14df61c1a1040638d5a7354ac117c6cc032330c1721c93eae4b2734b36e972ace8f2dfc3f71ca8bbf5654eb9876972e5ad92556b0724443c4f8003ae026c568c08cf4bef071b4fdd8b269d3db984451ae705893592dd3108db01c364994b19099d08c82706bab44e4a858d336b1fc1fbd2c8d1e5c1955249df0c7d7147032b9e51a96061427f569188f44fdf00ed0a4428dd3dadca3179b4c38e53a824d128ee564c144261858054972d4fceb237d86f4c1a3aab4d9c5e1101e6b07bcb51cddd46a62044768c732ab7cc9e57bd0c69392b2eb2f92dbb82047362edb8d3b47fde30d019bc4cf3b4cb4386ed03f322902b8444d56f91c25d22bcfd1e481452f1e4376422a22651c6d0219f8d5e02dd5b3a1886b4fcf125d42076366b854948b101ad106a964eebc7e3211857f179692854019f72380a190c669da0608f439f60d6d672a664ce15ab0718c27c14f3a185f19f5d8519bc6428a9fb3e1d947dd09def041a2d5f11a9a42fe667a2643d47c8e2364a32edd70d8c7fd8d69600433c0b2a0927f644e36cf17c46e91a02f620435a2b39d36136551af7e95e61a81714956b44c5be205d50dffe263c5969eb817f6a121dc9af13a0dc64dbd114a17e1013586ca12401d2c479b8e82802a4b56857166d1fb512da904d77bda27156f63e14185a2a2f19e367ad7236192b6578c220005b713d33507b1b64101f01e2311749f054f47ec5ca7ce9b8003183d539dd5752f109c1e870d0e8a0b6c70797bf100272f8438c9202bb7948fd5d68173aa68ed5b7321335e520c04a7cc4c6fb922fc5973679e924bab45f311bdb45663b0a99f4ba7dd1dd72c8efe0f87ffb99049d4869aba1fcbde93106dc02510a4f0a6d5b280f2da69bd65edbe5e5f63f27c1cadf97bf78529d7dea22859bcfc25cc79f652a17a04d535ffa15d550fe072730f434280e42c756d713c2dd4e51a51969589f720a5d84045e67fd656217613ff4e7abed7cbddd52d215dfbb5a651a6e195dc452cbd6729c20ff06b5d8fb0dcbdddd6728dd89a23a21ef59e3b7dd82c25aa41b385543de2fd42a23f7efe5f9307028cba608dff6890a776a7cb2c87408744572221131235b411b3cf0794754174e68301365de595547b230865e8e109579da0ca06bf19f514f5b574c0219eb7398890298db04604aaa054d96333461d67ff53e6d56a75aca5fdcb11403029a0d0551a76dad45d940806ca4527ec65dd8bf09e0398ad18c1efc63a763117a52b484c7d20ceeb404e24639069378455ee3706ba82a32098e5eb73b961b24abfe74b7d5dfd96bc1b3eb1bc650c802fe61c60b2b9adcdc12a8f372ed116824913cae25a355bfe40f3d7080d44c33b1c860a87b4ed496c6267254d84a0c6c42b93488498ac05aa1941fb82228c444b0dfa020d47cf87b4e9d0757dbc987995c586ee936ddca20044cce577d8cd91689f4a0676ad00af68bcb197ba85dd2dc0df31b532b207e475b7cef2d6f7bd39d29370cd52b87fc8afec6de3dd401ec3c8e755df9f07dfa174deecf68a3109b540622f29a7956b0fe573fe8ab0520d31761e0fa2b432a770aa4e51baf781a7bdd2a2b888cb8b3d7a3074553c47614a6e92d3e077b7a182197f9ffa30eb020fb61432d0f48232508876a067149c9f3c401aa83d593177c891064f8c3d080fedb46"
        )
    ];

    find_checkpoint(height, checkpoints)
}


fn get_main_checkpoint(height: u64) ->  Option<(u64, &'static str, &'static str)> {
    let checkpoints: Vec<(u64, &str, &str)> = vec![
        (570000, "0000000029e74486a37a0a778023e5a0ad3cf6e179df84559961339ab3850a73",
                 "004722388d0c9c3098438043c64f1c74d4145372141e608daff56a4779c9a5f557c5e6b11b3b539d1f070e11dda46de961ed727fa4b37f2aed51d6923f17ca22d21f387c56867bbc23067d3fef7485d57b52b9830874c62133ed9b3fb2f8145417ad695b961c3dafb11f285c6a1c15ac5dd963133d7ce0f499126291bb9223ca125b878bf282c7cfc5dfd5eb3fc9e9f719ab0c286e03d4f39f8929bd82350ce77e871f6a665bc05f04c5c87983c8ad42e7b7d5f514e3e546200cb80e750b8dbb151469ddcde8202935fa74e3da7177946c090f641a9c64d5aea57b9f846158ec9f6a9a829e1fe91696244fb30fccf57f61d7af28c0472355091f25e6097e9dad14473d3d0b21f5967a3a90dbb5709e338e1ba6c6ec4729ba535e7eb37f3df1a74ac5f41c22d215fc54e589ec0281e175e422204d7c6950ef53c9071fb2072464a16c9fc7b914f119b208697e3b6dccaf007d9ccc64c80319e87cb22ad0df67a0c124d8cfb308fbb4abb6ea63fbe227988f1bde0fe684381b0ed409145c6ead8c46b7206c521121551eca3429f2ff6a11a47fcaa6c9e614d833b43d5e77d2995d274cb8c8059cdfd820e352133c5c6165cb74e4fd9ff637c2da130bceb2c607719736a1c4632b3c5b5947492b94fb0a34d34f51366297d633e115d8b9fe5afe54f9bcca0e1ed3c8f48a068f1de332bb51cd3b01e65655af1f0152232fafa15ca35762451eab6408ee2c2295254d2941c99683909fc15eea9531f8d5c4dd892296f0f401b4027a2b08e627503c90f8f62245696bb23cf1b80a4080f6b150aa897495739561ea8bcce5e497f345036d0296a70cf627d2972148d376cb32c9ba3fce440c0d02cfc49b916f834fe5bd1ceda301c3dd34d5fd0d7775bd9a2952b7d7f16460b0cc0c69a2627d420e144cbdc2fe5ae2253e39a1cfb77d58891287941abf006faf107f6b8473a7f7858918badfa57bd7fbc49c123d24ec2f13748519793304c233679526275cee560b4aa18cbe28a9615242d2367ae894508f8716d71e0e316af01fb8ec41d654028bb3ff226cd59c8f779408b5a2be275ee58fc96bb3083e9e1ed7b1eb7f90b814fa4c948605bc069da084ee207e7d39976cfa349d214c94b5f18baf14ea4e7356dfd6fb6e3bca9e2969380be486daed657196b32476d0ccb681b05df58d2c01893bb4628fbe97c56ba11bee6c9980860bf974f810435577f2053881264b53b970331c25a28691298f14392e9f604ff0a57c9613920ece768dedecbb6fe62b9726c6b19fb1cd6dc37e3ccdf23b83b9ba1fc519066afff98a27157dc61197f349e3613339c0dbc9190bfc579c3052065d760aa22d1a12883cc36108b068091bf30ebf2c18e59ce6c1b959328910c02e31c2b228a4694daad44051668143d5b1e85195b05b7308df00ee43b4e561ff07be0c40d031cf34821bb9d530831498f3dddf9e9805334171c72f7b9c250d7c529c2d103ea314bf567c6f3c2a616310cfd14790489e6e85167795ea1b602d318b3baae621dd3602c9a47ba81c1c06ab7c98c8620f5b4dd1ed38fa52a9652399940a47c08d02bbab9d91c6f3a4d622fc56a2233d3b55dd3db6b2440a60c04f84c9e698ebdf3c75f68ffd5eb785cca767a9a59fe7a3220a25dedf6bcee923d9749501b5f2a2544111424ccd46cba1e2046b906cfced8a269442959d9fba4730a792afe8fdc939570578aec90c0be1260bc8f9d1e477e1d9e87eea3d1e9a4e6c42276c99ec6e1473db0ee5a71e79da83469752d781bc13651aa151b5ee4feb574159b2af843eac3019b5141ca0a60a7a1db1b7ceca4252811a24057b5d11a89c224bd2409325b771378f58b19c6c12c6ba3e7b6f913dbd6602dd9fa847aaa38479843f683df9e256d741"
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
        assert_eq!(get_main_checkpoint(570000).unwrap().0, 570000);
        assert_eq!(get_main_checkpoint(585000).unwrap().0, 570000);
    }

}