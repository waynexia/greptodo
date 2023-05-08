import axios from "axios"
import { TypeAnimation } from 'react-type-animation';

const code_example = `async fn some_files_impl(
  state: ServerState,
  query: SomeFilesQuery,
  form: SomeFilesQuery,
) -> FeedResult<SomeFilesResponse> {
  let org = query
      .org
      .or(form.org)
      .with_context(|| MissingParameterSnafu { param: "org" })?;
  let repo = query
      .repo
      .or(form.repo)
      .with_context(|| MissingParameterSnafu { param: "repo" })?;

  let repo_path = state.repo_path(&org, &repo);
  let mut entries = WalkDir::new(repo_path)
      .into_iter()
      .filter_map(|e| e.ok())
      .filter(|e| e.file_type().is_file())
      .filter(|e| {
          e.metadata().unwrap().len() >= MIN_FILE_SIZE
              && e.metadata().unwrap().len() <= MAX_FILE_SIZE
      })
      .map(|e| e.path().to_str().unwrap().to_string())
      .collect::<Vec<_>>();

  let mut rng = rand::thread_rng();
  entries.shuffle(&mut rng);
  let file_names = entries.into_iter().take(DEFAULT_FILE_NUM);

  let mut files = Vec::with_capacity(DEFAULT_FILE_NUM);
  for file_name in file_names {
      let content = std::fs::read_to_string(&file_name).context(FileSystemSnafu)?;
      files.push(content);
  }

  Ok(SomeFilesResponse {
      num_files: files.len(),
      files,
      ..Default::default()
  })
}
`

export default function Background(props) {
  axios.post('http://127.0.0.1:7531/api/some_files?org=waynexia&repo=unkai',
    {},
    { headers: { "Content-Type": "application/x-www-form-urlencoded" } }
  ).then(function (response) {
  });

  // todo: use the content from the response
  return (
    <pre className="overflow-clip no-scrollbar max-h-screen max-w-screen text-gray p-l-4">
      <TypeAnimation sequence={[code_example]} wrapper="span"></TypeAnimation>
    </pre>
  )
}