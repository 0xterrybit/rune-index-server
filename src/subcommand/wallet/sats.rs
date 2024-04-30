use super::*;

#[derive(Debug, Parser)]
pub(crate) struct Sats {
  #[arg(
    long,
    help = "Find satoshis listed in first column of tab-separated value file <TSV>."
  )]
  tsv: Option<PathBuf>,
}

#[derive(Serialize, Deserialize)]
pub struct OutputTsv {
  pub found: BTreeMap<String, SatPoint>,
  pub lost: BTreeSet<String>,
}

#[derive(Serialize, Deserialize)]
pub struct OutputRare {
  pub sat: Sat,
  pub output: OutPoint,
  pub offset: u64,
  pub rarity: Rarity,
}

impl Sats {
  pub(crate) fn run(&self, wallet: Wallet) -> SubcommandResult {
    ensure!(
      wallet.has_sat_index(),
      "sats requires index created with `--index-sats` flag"
    );

    let haystacks = wallet.get_output_sat_ranges()?;

    if let Some(path) = &self.tsv {
      let tsv = fs::read_to_string(path)
        .with_context(|| format!("I/O error reading `{}`", path.display()))?;

      let needles = Self::needles(&tsv)?;

      let found = Self::find(&needles, &haystacks);

      let lost = needles
        .into_iter()
        .filter(|(_sat, value)| !found.contains_key(*value))
        .map(|(_sat, value)| value.into())
        .collect();

      Ok(Some(Box::new(OutputTsv { found, lost })))
    } else {
      let mut output = Vec::new();
      for (outpoint, sat, offset, rarity) in Self::rare_sats(haystacks) {
        output.push(OutputRare {
          sat,
          output: outpoint,
          offset,
          rarity,
        });
      }
      Ok(Some(Box::new(output)))
    }
  }

  fn find(
    needles: &[(Sat, &str)],
    ranges: &[(OutPoint, Vec<(u64, u64)>)],
  ) -> BTreeMap<String, SatPoint> {
    let mut haystacks = Vec::new();

    for (outpoint, ranges) in ranges {
      let mut offset = 0;
      for (start, end) in ranges {
        haystacks.push((start, end, offset, outpoint));
        offset += end - start;
      }
    }

    haystacks.sort_by_key(|(start, _, _, _)| *start);

    let mut i = 0;
    let mut j = 0;
    let mut results = BTreeMap::new();
    while i < needles.len() && j < haystacks.len() {
      let (needle, value) = needles[i];
      let (&start, &end, offset, outpoint) = haystacks[j];

      if needle >= start && needle < end {
        results.insert(
          value.into(),
          SatPoint {
            outpoint: *outpoint,
            offset: offset + needle.0 - start,
          },
        );
      }

      if needle >= end {
        j += 1;
      } else {
        i += 1;
      }
    }

    results
  }

  fn needles(tsv: &str) -> Result<Vec<(Sat, &str)>> {
    let mut needles = tsv
      .lines()
      .enumerate()
      .filter(|(_i, line)| !line.starts_with('#') && !line.is_empty())
      .filter_map(|(i, line)| {
        line.split('\t').next().map(|value| {
          Sat::from_str(value).map(|sat| (sat, value)).map_err(|err| {
            anyhow!(
              "failed to parse sat from string \"{value}\" on line {}: {err}",
              i + 1,
            )
          })
        })
      })
      .collect::<Result<Vec<(Sat, &str)>>>()?;

    needles.sort();

    Ok(needles)
  }

  fn rare_sats(haystacks: Vec<(OutPoint, Vec<(u64, u64)>)>) -> Vec<(OutPoint, Sat, u64, Rarity)> {
    haystacks
      .into_iter()
      .flat_map(|(outpoint, sat_ranges)| {
        let mut offset = 0;
        sat_ranges.into_iter().filter_map(move |(start, end)| {
          let sat = Sat(start);
          let rarity = sat.rarity();
          let start_offset = offset;
          offset += end - start;
          if rarity > Rarity::Common {
            Some((outpoint, sat, start_offset, rarity))
          } else {
            None
          }
        })
      })
      .collect()
  }
}
 