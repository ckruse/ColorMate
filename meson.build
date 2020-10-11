project('colormate',
          version: '0.1.4',
    meson_version: '>= 0.50.0',
  default_options: [ 'warning_level=2',
                   ],
)

i18n = import('i18n')


subdir('data')
subdir('src')
subdir('po')

meson.add_install_script('build-aux/meson/postinstall.py')
meson.add_dist_script(
  'build-aux/vendor.sh',
  meson.build_root() / 'meson-dist' / meson.project_name() + '-' + meson.project_version(),
  meson.source_root()
)