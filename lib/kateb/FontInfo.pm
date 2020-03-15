package kateb::FontInfo;
$kateb::FontInfo::VERSION = '01.00.04';

use strict;
use warnings;

sub new {
    my $class  = shift;
    my $self = _sources();
    bless $self, $class;

    return $self;
}

sub _sources {
	my $github_apis = {
		vazir => {
			name => 'vazir-font',
			api => 'https://api.github.com/repos/rastikerdar/vazir-font/tags',
			publisher => 'https://github.com/rastikerdar'
		},
		samim => {
			name => 'samim-font',
			api => 'https://api.github.com/repos/rastikerdar/samim-font/tags',
			publisher => 'https://github.com/rastikerdar'
		},
		tanha => {
			name => 'tanha-font',
			api => 'https://api.github.com/repos/rastikerdar/tanha-font/tags',
			publisher => 'https://github.com/rastikerdar'
		},
		shabnam => {
			name => 'shabnam-font',
			api => 'https://api.github.com/repos/rastikerdar/shabnam-font/tags',
			publisher => 'https://github.com/rastikerdar'
		},
		gandom => {
			name => 'gandom-font',
			api => 'https://api.github.com/repos/rastikerdar/gandom-font/tags',
			publisher => 'https://github.com/rastikerdar'
		},
		parastoo => {
			name => 'parastoo-font',
			api => 'https://api.github.com/repos/rastikerdar/parastoo-font/tags',
			publisher => 'https://github.com/rastikerdar'
		},
		sahel => {
			name => 'sahel-font',
			api => 'https://api.github.com/repos/rastikerdar/sahel-font/tags',
			publisher => 'https://github.com/rastikerdar'
		},
		'vazir-code' => {
			name => 'vazir-code-font',
			api => 'https://api.github.com/repos/rastikerdar/vazir-code-font/tags',
			publisher => 'https://github.com/rastikerdar'
		},
		nahid => {
			name => 'nahid-font',
			api => 'https://api.github.com/repos/rastikerdar/nahid-font/tags',
			publisher => 'https://github.com/rastikerdar'
		},
		mikhak => {
			name => 'Mikhak',
			api => 'https://api.github.com/repos/aminabedi68/Mikhak/tags',
			publisher => 'https://github.com/aminabedi68'
		},
		ganjnameh => {
			name => 'GanjnamehFont',
			api => 'https://api.github.com/repos/font-store/GanjnamehFont/tags',
			publisher => 'https://github.com/font-store'
		},
		behdad => {
			name => 'BehdadFont',
			api => 'https://api.github.com/repos/font-store/BehdadFont/tags',
			publisher => 'https://github.com/font-store'
		},
		nika => {
			name => 'NikaFont',
			api => 'https://api.github.com/repos/font-store/NikaFont/tags',
			publisher => 'https://github.com/font-store'
		},
		farbod => {
			name => 'FarbodFont',
			api => 'https://api.github.com/repos/font-store/FarbodFont/tags',
			publisher => 'https://github.com/font-store'
		},
		pfont => {
			name => 'pfont',
			api => 'https://api.github.com/repos/pfont/pfont/tags',
			publisher => 'https://github.com/pfont'
		}
	};
	return $github_apis;
}

sub vazir {
	my $self    = shift;
	my $version = shift;
	my $link = 
		$self->{vazir}->{publisher} . "/" .
		$self->{vazir}->{name} .
		"/releases/download/" .
		$version . "/" .
		$self->{vazir}->{name} . "-" . $version .
		".zip"
	;
	return $link;
}

sub samim {
	my $self    = shift;
	my $version = shift;
	my $link = 
		$self->{samim}->{publisher} . "/" .
		$self->{samim}->{name} .
		"/releases/download/" .
		$version . "/" .
		$self->{samim}->{name} . "-" . $version .
		".zip"
	;
	return $link;
}

sub tanha {
	my $self    = shift;
	my $version = shift;
	my $link = 
		$self->{tanha}->{publisher} . "/" .
		$self->{tanha}->{name} .
		"/releases/download/" .
		$version . "/" .
		$self->{tanha}->{name} . "-" . $version .
		".zip"
	;
	return $link;
}

sub shabnam {
	my $self    = shift;
	my $version = shift;
	my $link = 
		$self->{shabnam}->{publisher} . "/" .
		$self->{shabnam}->{name} .
		"/releases/download/" .
		$version . "/" .
		$self->{shabnam}->{name} . "-" . $version .
		".zip"
	;
	return $link;
}

sub gandom {
	my $self    = shift;
	my $version = shift;
	my $link = 
		$self->{gandom}->{publisher} . "/" .
		$self->{gandom}->{name} .
		"/releases/download/" .
		$version . "/" .
		$self->{gandom}->{name} . "-" . $version .
		".zip"
	;
	return $link;
}

sub parastoo {
	my $self    = shift;
	my $version = shift;
	my $link = 
		$self->{parastoo}->{publisher} . "/" .
		$self->{parastoo}->{name} .
		"/releases/download/" .
		$version . "/" .
		$self->{parastoo}->{name} . "-" . $version .
		".zip"
	;
	return $link;
}

sub sahel {
	my $self    = shift;
	my $version = shift;
	my $link = 
		$self->{sahel}->{publisher} . "/" .
		$self->{sahel}->{name} .
		"/releases/download/" .
		$version . "/" .
		$self->{sahel}->{name} . "-" . $version .
		".zip"
	;
	return $link;
}

sub vazir_code {
	my $self    = shift;
	my $version = shift;
	my $link = 
		$self->{vazir_code}->{publisher} . "/" .
		$self->{vazir_code}->{name} .
		"/releases/download/" .
		$version . "/" .
		$self->{vazir_code}->{name} . "-" . $version .
		".zip"
	;
	return $link;
}

sub nahid {
	my $self    = shift;
	my $version = shift;
	my $link = 
		$self->{nahid}->{publisher} . "/" .
		$self->{nahid}->{name} .
		"/releases/download/" .
		$version . "/" .
		$self->{nahid}->{name} . "-" . $version .
		".zip"
	;
	return $link;
}

sub mikhak {
	my $self    = shift;
	my $version = shift;
	my $link = 
		$self->{mikhak}->{publisher} . "/" .
		$self->{mikhak}->{name} .
		"/releases/download/" .
		$version . "/" .
		$self->{mikhak}->{name} . "-" . $version .
		".zip"
	;
	return $link;
}

sub ganjnameh {
	my $self    = shift;
	my $version = shift;
	my $ver = $version;
	$ver =~ s/^v//;
	my $link = 
		$self->{ganjnameh}->{publisher} . "/" .
		$self->{ganjnameh}->{name} .
		"/releases/download/" .
		$version . "/" .
		"pack." . $ver .
		".zip"
	;
	return $link;
}

sub behdad {
	my $self    = shift;
	my $version = shift;
	my $ver = $version;
	$ver =~ s/^v//;
	my $link = 
		$self->{behdad}->{publisher} . "/" .
		$self->{behdad}->{name} .
		"/releases/download/" .
		$version . "/" .
		"Behdad-" . $ver .
		".zip"
	;
	return $link;
}

sub nika {
	my $self    = shift;
	my $version = shift;
	my $link = 
		$self->{nika}->{publisher} . "/" .
		$self->{nika}->{name} .
		"/releases/download/" .
		$version . "/" .
		"nika." . $version .
		".zip"
	;
	return $link;
}

sub farbod {
	my $self    = shift;
	my $version = shift;
	my $ver = $version;
	$ver =~ s/^v//;
	my $link = 
		$self->{farbod}->{publisher} . "/" .
		$self->{farbod}->{name} .
		"/releases/download/" .
		$version . "/" .
		"Farbod-" . $ver .
		".zip"
	;
	return $link;
}

sub pfont {
	my $self    = shift;
	my $version = shift;
	my $ver = $version;
	my $link = 
		$self->{pfont}->{publisher} . "/" .
		$self->{pfont}->{name} .
		"/releases/download/" .
		$version . "/" .
		"pfont.zip"
	;
	return $link;
}

1;


