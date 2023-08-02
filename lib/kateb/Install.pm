package kateb::Install;
$kateb::Install::VERSION = '1.1.0';

use strict;
use warnings;
use 5.012;

use kateb::FontInfo;
use JSON::PP;
use HTTP::Tinyish;
use URI            ();
use File::Spec::Functions qw(catdir catfile tmpdir);
use Archive::Zip qw( :ERROR_CODES :CONSTANTS );
use File::Copy;
use Term::ANSIColor qw(:constants);

my $shown;
my %c =
(
	byellow => (BOLD YELLOW),
	bpurple => (BOLD MAGENTA),
	bblue   => (BOLD BLUE),
	bgreen  => (BOLD GREEN),
	bold    => (BOLD),
	bred    => (BOLD RED),
	bcyan   => (BOLD CYAN),
	reset   => (RESET),
);

sub new {
	my $class = shift;
	my @args  = @_;
	my $self;
	if (not $args[0])
	{
		$self = {
			error   => 1,
			message => "type font name(s) or use all|-a option",
			errlist => [],
			fonts   => '',
		};
	} elsif ($args[0] =~ /^all$|^-a$/)
	{
		$self = {
			error   => 0,
			message => "ok!",
			errlist => [],
			fonts   => _all_fonts(),
		};
	} else
	{
		my @compare = _compare(@args);
		if (@compare)
		{
			$self = {
				error   => 2,
				message => "unknown font(s)",
				errlist => \@compare,
				fonts   => '',
			};
		} else
		{
			$self = {
				error   => 0,
				message => "ok!",
				errlist => [],
				fonts   => \@args,
			};
		}
	}
	bless $self, $class;

	return $self;
}

sub install {
	my $self       = shift;
	my $local_data = shift;
	my @fonts      = @{$self->{fonts}};
	my $info       = kateb::FontInfo->new;

	my $json_file  = $local_data->{jsonFile};
	my @check;

	foreach my $font_name (@fonts)
	{
		my $version = _online_version($info->{$font_name}->{api});
		if ($local_data->{installedVersions}->{$font_name})
		{
			printf("$c{bgreen}%10s$c{reset}",$font_name);
			say " version " . $c{bgreen} . $version . $c{reset} . " is already installed";

			unless ($version eq $local_data->{installedVersions}->{$font_name}) {
				push @check, $font_name;
				say "\tnew version is available: " .
				$c{bred} . $version . $c{reset};
			}
			next;
		}
		_do($font_name, $local_data, $version);
	}
	kateb::LocalData->write_data($local_data->{installedVersions}, $json_file);
	say "\nto update fonts:
	$c{bblue}kateb update @check$c{reset}" if @check;

	# my @fc_cache = ("fc-cache", "-f", "-v", $local_data->{targetDir}, ">/dev/null");
	# qx(@fc_cache);
	system("fc-cache -f -v $local_data->{targetDir} >/dev/null");
}

sub update {
	my $self       = shift;
	my $local_data = shift;
	my @fonts      = @{$self->{fonts}};
	my $info       = kateb::FontInfo->new;

	my $json_file  = $local_data->{jsonFile};

	foreach my $font_name (@fonts)
	{
		if (not $local_data->{installedVersions}->{$font_name})
		{
			printf("$c{bred}%10s$c{reset}",$font_name);
			print " is not already installed, but can be installed with:\n\n";
			say $c{bblue} . "\tkateb install $font_name\n". $c{reset};
			next;
		}

		my $version = _online_version($info->{$font_name}->{api});

		if ($version eq $local_data->{installedVersions}->{$font_name})
		{
			printf("$c{bgreen}%10s$c{reset}",$font_name);
			print ", The latest version is already installed: ";
			say $c{bgreen} . $version . $c{reset};
			next;
		}
		_do($font_name, $local_data, $version);
	}
	kateb::LocalData->write_data($local_data->{installedVersions}, $json_file);

	# my @fc_cache = ("fc-cache", "-f", "-v", $local_data->{targetDir}, ">/dev/null");
	# qx(@fc_cache);
	system("fc-cache -f -v $local_data->{targetDir} >/dev/null");
}

sub reinstall {
	my $self       = shift;
	my $local_data = shift;
	my @fonts      = @{$self->{fonts}};
	my $info       = kateb::FontInfo->new;

	my $json_file  = $local_data->{jsonFile};

	foreach my $font_name (@fonts)
	{
		if (not $local_data->{installedVersions}->{$font_name})
		{
			printf("$c{bred}%10s$c{reset}",$font_name);
			print " is not already installed, but can be installed with:\n\n";
			say $c{bblue} . "\tkateb install $font_name\n". $c{reset};
			next;
		}
		my $version = _online_version($info->{$font_name}->{api});
		_do($font_name, $local_data, $version);
	}
	kateb::LocalData->write_data($local_data->{installedVersions}, $json_file);

	# my @fc_cache = ("fc-cache", "-f", "-v", $local_data->{targetDir}, ">/dev/null");
	# qx(@fc_cache);
	system("fc-cache -f -v $local_data->{targetDir} >/dev/null");
}

############################################
######  do: install/reinstall/update  ######
############################################

sub _do {
	my $font_name  = shift;
	my $local_data = shift;
	my $version    = shift;
	my $temp_dir   = $local_data->{tempDir};
	my $cache_dir  = $local_data->{cacheDir};
	my $target_dir = $local_data->{targetDir};

	my $info       = kateb::FontInfo->new;
	print "working on $font_name-$version...";

	if ($font_name eq 'lalezar')
	{
		my $url = $info->$font_name($version);
		my $archive_file = catfile($cache_dir, "Lalezar-Regular.ttf");
		_download( $url, $archive_file ) ? print "\t[Downloaded]\n" : print "\t[Failed]\n";
		_copy_fonts( $target_dir, $archive_file );
		$local_data->{installedVersions}->{$font_name} = $version;
		say $c{bgreen} . $font_name . $c{reset} . " installed. version: " . $c{bgreen} . $version . $c{reset};
	} elsif ($font_name eq 'nastaliq')
	{
		my $url = $info->$font_name($version);
		my $archive_file = catfile($cache_dir, "IranNastaliq-Web.ttf");
		_download( $url, $archive_file ) ? print "\t[Downloaded]\n" : print "\t[Failed]\n";
		_copy_fonts( $target_dir, $archive_file );
		$local_data->{installedVersions}->{$font_name} = $version;
		say $c{bgreen} . $font_name . $c{reset} . " installed. version: " . $c{bgreen} . $version . $c{reset};
	} else
	{
		my $url = $info->$font_name($version);
		my $archive_file = catfile($temp_dir, "$font_name-$version.zip");
		_download( $url, $archive_file ) ? print "\t[Downloaded]\n" : print "\t[Failed]\n";
		my @extracted_fonts = _unzip($font_name, $archive_file, $cache_dir);
		$local_data->{installedVersions}->{$font_name} = $version;
		_copy_fonts($target_dir, @extracted_fonts) if @extracted_fonts;
		say $c{bgreen} . $font_name . $c{reset} . " installed. version: " . $c{bgreen} . $version . $c{reset};
	}

	if ($font_name eq 'vazir')
	{
		my @vazirs = grep { m"Vazir(?!(?:-Code|matn))"g } glob catfile($target_dir, "*.ttf");
		unlink foreach @vazirs;
	}
}

############################################

sub _compare {
	my @args = @_;
	my $info = kateb::FontInfo->new();
	my @compare;
	foreach my $font_name (@args)
	{
		if (not $info->{$font_name})
		{
			push @compare, $font_name;
		}
	}
	return @compare;
}

sub _all_fonts {
	my $info = kateb::FontInfo->new();
	my @all_fonts = keys %{$info};
	return \@all_fonts;
}

sub _online_version {
	my $api = shift;

	my $http = HTTP::Tinyish->new
	(
		agent => "kateb/$kateb::VERSION",
		verify_SSL => 1
	);
	my $res = $http->get($api);

	my $tags = decode_json( $res->{content} );
	my $version;
	eval
	{
		$version = $tags->[0]->{name};
	}; if ($@)
	{
		say "$c{bred}github API rate limit exceeded. This limit is 50 times per hour, plz try again in about an hour$c{reset}";
		exit;
	}
	return $version;
}

sub _copy_fonts {
	my $target_dir      = shift;
	my @extracted_fonts = @_;

	#my $glob = catfile($cache_dir, "*.ttf");
	#my @downloaded_fonts = glob $glob;
	foreach my $font_file (@extracted_fonts)
	{
		my ($volume, $directories, $file) = File::Spec->splitpath( $font_file );
		my $target = catfile($target_dir, $file);
		##########################################################
		#   say "...copy $file to $target_dir";
		##########################################################

		copy($font_file, $target);
	}
}

###################################
############  unZip  ##############
###################################

sub _unzip {
	my $font_name    = shift;
	my $archive_file = shift;
	my $cache_dir    = shift;
	my @extracted_fonts;
	my $zip = Archive::Zip->new();
	unless ( $zip->read( $archive_file ) == AZ_OK )
	{
		die 'read error';
	}
	# unzip fonts
	if ($font_name eq 'parastoo')
	{
		foreach my $file (grep { m{(web/((?!/).)*\.ttf$)|(print/((?!/).)*\.ttf$)}g } $zip->memberNames())
		{
			my ($volume, $directories, $file_name) = File::Spec->splitpath($file);
			$zip->extractMember( $file, catfile($cache_dir, $file_name) );
			push @extracted_fonts, catfile($cache_dir, $file_name);
		}
	} elsif ($font_name =~ /^ganjnameh$|^nika$|^mikhak$|^shahab$/)
	{
		foreach my $file (grep { m{.*\.ttf$}g } $zip->memberNames())
		{
			my ($volume, $directories, $file_name) = File::Spec->splitpath($file);
			$zip->extractMember( $file, catfile($cache_dir, $file_name) );
			push @extracted_fonts, catfile($cache_dir, $file_name);
		}
	} elsif ($font_name =~ /^pfont$/)
	{
		foreach my $file (grep { m{(?<!__MACOSX\/)pfont/ttf/Hinted/.*\.ttf$}g } $zip->memberNames())
		{
			my ($volume, $directories, $file_name) = File::Spec->splitpath($file);
			$zip->extractMember( $file, catfile($cache_dir, $file_name) );
			push @extracted_fonts, catfile($cache_dir, $file_name);
		}
	} elsif ($font_name =~ /^estedad$/)
	{
		foreach my $file (grep { m{Static/ttf/.*\.ttf$}g } $zip->memberNames())
		{
			my ($volume, $directories, $file_name) = File::Spec->splitpath($file);
			$zip->extractMember( $file, catfile($cache_dir, $file_name) );
			push @extracted_fonts, catfile($cache_dir, $file_name);
		}
	} elsif ($font_name =~ /^vazir$/)
	{
		foreach my $file (grep { m"^fonts/ttf/((?!/).)*\.ttf$"g } $zip->memberNames())
		{
			my ($volume, $directories, $file_name) = File::Spec->splitpath($file);
			$zip->extractMember( $file, catfile($cache_dir, $file_name) );
			push @extracted_fonts, catfile($cache_dir, $file_name);
		}
	}
	else
	{
		foreach my $file (grep { m"^((?!/).)*\.ttf$"g } $zip->memberNames())
		{
			$zip->extractMember( $file, catfile($cache_dir, $file) );
			push @extracted_fonts, catfile($cache_dir, $file);
		}
	}
	return @extracted_fonts;
}

###################################
############ Download #############
###################################

sub _download {
	my $url          = shift;
	my $archive_file = shift;
	unlink $archive_file if -e $archive_file;
	# my $file = $archive_file;
	# $file =~ s{^.*/([^/]*\.zip)}{$1};

	$url = URI->new($url);
	my $http = HTTP::Tinyish->new
	(
		agent => "kateb/$kateb::VERSION",
		verify_SSL => 1
	);
	my $res = $http->mirror($url, $archive_file);
	$res->{success} ? return 1 : return 0;
}

1;
