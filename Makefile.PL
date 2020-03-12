use strict;
use warnings;
use utf8;

use 5.016;

use ExtUtils::MakeMaker;

WriteMakefile(
    NAME               => 'kateb',
    AUTHOR             => 'Kiavash',
    VERSION_FROM       => 'lib/kateb.pm',
    ABSTRACT           => 'indtsll and update Farsi free fonts',
    LICENSE            => 'gpl_3',
    EXE_FILES          => ["bin/kateb"],
    MIN_PERL_VERSION   => 5.016,
    CONFIGURE_REQUIRES => {
        'ExtUtils::MakeMaker' => 0,
    },
    PREREQ_PM => {
        'File::Path'            => 0,
        'File::Spec::Functions' => 0,
        'LWP::UserAgent'        => 0,
        'URI'                   => 0,
        'HTTP::Date'            => 0,
        'JSON::PP'              => 0,
        'JSON::MaybeXS'         => 0,
        'Archive::Zip'          => 0,
        'File::Copy'            => 0,
        'Term::ANSIColor'       => 0
    },
    dist       => { COMPRESS => 'gzip -9f', SUFFIX => 'gz', },
    META_MERGE => {
        'meta-spec' => { version => 2 },
        resources   => {
            repository => {
                type => 'git',
                url  => 'https://github.com/kiamazi/kateb.git',
                web  => 'https://github.com/kiamazi/kateb',
            },
        },
    },
);

1;
