project(
  'utena',
  'rust',
  version: '0.4.5',
  license: 'MIT',
  meson_version: '>= 0.59'
)

cargo = find_program('cargo', required: true)
rustc = find_program('rustc', required: true)

if get_option('debug')
  command = [
    cargo, 'build',
    '&&',
    'cp', meson.global_source_root() / 'target' / 'debug' / meson.project_name(), '@OUTPUT@'
  ]
else
  command = [
    cargo, 'build',
    '--release',
    '&&',
    'cp', meson.global_source_root() / 'target' / 'release' / meson.project_name(), '@OUTPUT@'
  ]
endif

prefix = get_option('prefix')

bindir = prefix / get_option('bindir')

utena_target = custom_target('utena',
  output: 'utena',
  build_by_default: true,
  install: true,
  install_dir: bindir,
  console: true,
  command: command
)


