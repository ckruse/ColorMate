colormate_sources += [
  'main.vala',
  'Application.vala',
  'Window.vala',
]

colormate_deps = [
  dependency('gio-2.0', version: '>= 2.50'),
  dependency('gtk+-3.0', version: '>= 3.22'),
  dependency('granite', version: '>=6.0.0'),
  meson.get_compiler('c').find_library('m')
]

executable('colormate', colormate_sources,
  vala_args: '--target-glib=2.50',  dependencies: colormate_deps,
  install: true,
)
