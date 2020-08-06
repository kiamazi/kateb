package kateb::FontInfo;
$kateb::FontInfo::VERSION = '01.00.18';

use strict;
use warnings;

sub new {
	my $class  = shift;
	my $self   = _sources();
	bless $self, $class;

	return $self;
}

sub _sources {
	my $github_apis = {
		vazir => {
			name => 'vazir-font',
			api => 'https://api.github.com/repos/rastikerdar/vazir-font/tags',
			publisher => 'https://github.com/rastikerdar',
			repo => 'https://github.com/rastikerdar/vazir-font/',
			publisher_name => 'Saber Rastikerdar'
		},
		samim => {
			name => 'samim-font',
			api => 'https://api.github.com/repos/rastikerdar/samim-font/tags',
			publisher => 'https://github.com/rastikerdar',
			repo => 'https://github.com/rastikerdar/samin-font/',
			publisher_name => 'Saber Rastikerdar'
		},
		tanha => {
			name => 'tanha-font',
			api => 'https://api.github.com/repos/rastikerdar/tanha-font/tags',
			publisher => 'https://github.com/rastikerdar',
			repo => 'https://github.com/rastikerdar/tanha-font/',
			publisher_name => 'Saber Rastikerdar'
		},
		shabnam => {
			name => 'shabnam-font',
			api => 'https://api.github.com/repos/rastikerdar/shabnam-font/tags',
			publisher => 'https://github.com/rastikerdar',
			repo => 'https://github.com/rastikerdar/shabnam-font/',
			publisher_name => 'Saber Rastikerdar'
		},
		gandom => {
			name => 'gandom-font',
			api => 'https://api.github.com/repos/rastikerdar/gandom-font/tags',
			publisher => 'https://github.com/rastikerdar',
			repo => 'https://github.com/rastikerdar/gandom-font/',
			publisher_name => 'Saber Rastikerdar'
		},
		parastoo => {
			name => 'parastoo-font',
			api => 'https://api.github.com/repos/rastikerdar/parastoo-font/tags',
			publisher => 'https://github.com/rastikerdar',
			repo => 'https://github.com/rastikerdar/parastoo-font/',
			publisher_name => 'Saber Rastikerdar'
		},
		sahel => {
			name => 'sahel-font',
			api => 'https://api.github.com/repos/rastikerdar/sahel-font/tags',
			publisher => 'https://github.com/rastikerdar',
			repo => 'https://github.com/rastikerdar/sahel-font/',
			publisher_name => 'Saber Rastikerdar'
		},
		vazircode => {
			name => 'vazir-code-font',
			api => 'https://api.github.com/repos/rastikerdar/vazir-code-font/tags',
			publisher => 'https://github.com/rastikerdar',
			repo => 'https://github.com/rastikerdar/vazir-code-font/',
			publisher_name => 'Saber Rastikerdar'
		},
		zira => {
			name => 'zira-code-font',
			api => 'https://api.github.com/repos/kiamazi/ziar-code-font/tags',
			publisher => 'https://github.com/kiamazi',
			repo => 'https://github.com/kiamazi/zira-code-font/',
			publisher_name => 'Kiavash Mazi
FiraCode + VazirCode'
		},
		nahid => {
			name => 'nahid-font',
			api => 'https://api.github.com/repos/rastikerdar/nahid-font/tags',
			publisher => 'https://github.com/rastikerdar',
			repo => 'https://github.com/rastikerdar/nahid-font/',
			publisher_name => 'Saber Rastikerdar'
		},
		mikhak => {
			name => 'Mikhak',
			api => 'https://api.github.com/repos/aminabedi68/Mikhak/tags',
			publisher => 'https://github.com/aminabedi68',
			repo => 'https://github.com/aminabedi68/Mikhak/',
			publisher_name => 'Amin Abedi'
		},
		estedad => {
			name => 'Estedad',
			api => 'https://api.github.com/repos/aminabedi68/Estedad/tags',
			publisher => 'https://github.com/aminabedi68',
			repo => 'https://github.com/aminabedi68/Estedad/',
			publisher_name => 'Amin Abedi'
		},
		ganjnameh => {
			name => 'GanjnamehFont',
			api => 'https://api.github.com/repos/font-store/GanjnamehFont/tags',
			publisher => 'https://github.com/font-store',
			repo => 'https://github.com/font-store/GanjnamehFont/',
			publisher_name => 'Saleh Souzanchi'
		},
		behdad => {
			name => 'BehdadFont',
			api => 'https://api.github.com/repos/font-store/BehdadFont/tags',
			publisher => 'https://github.com/font-store',
			repo => 'https://github.com/font-store/BehdadFont/',
			publisher_name => 'Saleh Souzanchi'
		},
		nika => {
			name => 'NikaFont',
			api => 'https://api.github.com/repos/font-store/NikaFont/tags',
			publisher => 'https://github.com/font-store',
			repo => 'https://github.com/font-store/NikaFont/',
			publisher_name => 'Saleh Souzanchi'
		},
		farbod => {
			name => 'FarbodFont',
			api => 'https://api.github.com/repos/font-store/FarbodFont/tags',
			publisher => 'https://github.com/font-store',
			repo => 'https://github.com/font-store/FarbodFont',
			publisher_name => 'Saleh Souzanchi'
		},
		shahab => {
			name => 'ShahabFont',
			api => 'https://api.github.com/repos/font-store/ShahabFont/tags',
			publisher => 'https://github.com/font-store',
			repo => 'https://github.com/font-store/ShahabFont',
			publisher_name => 'Saleh Souzanchi'
		},
		noon => {
			name => 'NoonFont',
			api => 'https://api.github.com/repos/font-store/NoonFont/tags',
			publisher => 'https://github.com/font-store',
			repo => 'https://github.com/font-store/NoonFont',
			publisher_name => 'Saleh Souzanchi'
		},
		pfont => {
			name => 'pfont',
			api => 'https://api.github.com/repos/pfont/pfont/tags',
			publisher => 'https://github.com/pfont',
			repo => 'https://github.com/pfont/pfont/',
			publisher_name => 'Persian Free Font'
		},
		lalezar => {
			name => 'Lalezar',
			api => 'https://api.github.com/repos/BornaIz/Lalezar/tags',
			publisher => 'https://github.com/BornaIz',
			repo => 'https://github.com/BornaIz/Lalezar/',
			publisher_name => 'Borna Izadpanah'
		}
	};
	return $github_apis;
}

sub lalezar {
	my $link    =
        'https://raw.githubusercontent.com/BornaIz/Lalezar/master/fonts/Lalezar-Regular.ttf';
	return $link;
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

sub vazircode {
	my $self    = shift;
	my $version = shift;
	my $link =
		$self->{vazircode}->{publisher} . "/" .
		$self->{vazircode}->{name} .
		"/releases/download/" .
		$version . "/" .
		$self->{vazircode}->{name} . "-" . $version .
		".zip"
	;
	return $link;
}

sub zira {
	my $self    = shift;
	my $version = shift;
	my $link =
		$self->{zira}->{publisher} . "/" .
		$self->{zira}->{name} .
		"/releases/download/" .
		$version . "/" .
		$self->{zira}->{name} . "-" . $version .
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

sub estedad {
	my $self    = shift;
	my $version = shift;
	my $link =
		$self->{estedad}->{publisher} . "/" .
		$self->{estedad}->{name} .
		"/releases/download/" .
		$version . "/" .
		$self->{estedad}->{name} . ".V" . $version .
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
	my $link =
		$self->{pfont}->{publisher} . "/" .
		$self->{pfont}->{name} .
		"/releases/download/" .
		$version . "/" .
		"pfont.zip"
	;
	return $link;
}

sub shahab {
	my $self    = shift;
	my $version = shift;
	my $ver = $version;
	$ver =~ s/^v//;
	my $link =
		$self->{shahab}->{publisher} . "/" .
		$self->{shahab}->{name} .
		"/releases/download/" .
		$version . "/" .
		"pack." . $ver .
		".zip"
	;
	return $link;
}

sub noon {
	my $self    = shift;
	my $version = shift;
	my $ver = $version;
	$ver =~ s/\./\-/g;
	my $link =
		$self->{noon}->{publisher} . "/" .
		$self->{noon}->{name} .
		"/releases/download/" .
		$version . "/" .
		"NOON_" . $ver .
		".zip"
	;
	return $link;
}

1;
