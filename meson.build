project('vibrant', 'rust',
          version: '0.1.0',
    meson_version: '>= 0.62.0',
  default_options: [ 'warning_level=2', 'werror=false', ],
)

i18n = import('i18n')
gnome = import('gnome')

project_id = 'com.felipekinoshita.Vibrant'

# Profiles
application_id = project_id
if get_option('profile') == 'development'
    profile = 'Devel'
    application_id = '@0@.Devel'.format(application_id)
    message(application_id)

    find_program('git')
    rev_txt = run_command('git', 'rev-parse', '--short', 'HEAD').stdout().strip()
    version = meson.project_version() + '-' + rev_txt
else
    message(application_id)
    profile = ''
    version = meson.project_version()
endif

subdir('data')
subdir('src')
subdir('po')

gnome.post_install(
     glib_compile_schemas: true,
    gtk_update_icon_cache: true,
  update_desktop_database: true,
)
