package kateb::LocalData;
$kateb::LocalData::VERSION = '01.00.26';

use strict;
use warnings;
use 5.012;

use kateb::FontInfo;
use JSON::PP;
use File::Temp;
use File::Path qw(make_path rmtree);
use File::Spec;
use File::Spec::Functions qw(catdir catfile tmpdir);

sub new {
	my $class  = shift;
	my $self = _prepare();
	bless $self, $class;

	return $self;
}

sub _prepare {
	my $execname = "kateb";

	#----------------------- GLOBAL VARIABLES -----------------------#
	# crate empty envs
	my $json_file;

	# Home directory
	my $home_dir = $ENV{HOME} || $ENV{LOGDIR} || (getpwuid $<)[7] || `echo -n ~`;

	# Configuration directory
	my $config_dir = catdir($ENV{XDG_CONFIG_HOME} || catdir($home_dir, '.config'), 	$execname);

	# download fonts directory
	my $cache_font_dir = catdir($config_dir, 'fonts');

	# user font directory [target directory]
	my $root_font_dir = catdir('/', 'usr', 'share', 'fonts', 'truetype', 'farsifreefont');
	my $user_font_dir = catdir($home_dir, '.local', 'share', 'fonts', 'farsifreefont');
	my $target_dir = $> == 0 ? $root_font_dir : $user_font_dir;


	# temp dir for downloads
	my $temp_dir = File::Temp->newdir();
	$temp_dir = catdir($temp_dir, 'kateb');
	make_path $temp_dir;

	# Configuration file
	# my $config_file = catfile($config_dir, "$execname.conf");

	# json Database file
	my $json_data_file = catfile($config_dir, "$execname.json");

	# check for exist config dir
	if (not -d $config_dir)
	{
		make_path($config_dir)
		  or note(":: Unable to create directory <<$config_dir>>: {!$!!}");
	}

	# check for exist taget directory
	if (not -d $target_dir)
	{
		make_path($target_dir)
		  or note(":: Unable to create directory <<$target_dir>>: {!$!!}");
	}

	# check json data base file
	my $installed_versions;

	if (not -e $json_data_file)
	{
		_reset_json_file($json_data_file);
		$json_file = _read_file($json_data_file);
		$installed_versions = decode_json($json_file);
	} else
	{
		$json_file = _read_file($json_data_file);
		eval
		{
			$installed_versions = decode_json($json_file)
		}; if ($@)
		{
			_reset_json_file($json_data_file);
			$json_file = _read_file($json_file);
			$installed_versions = decode_json($json_file);
		}
	}
	return
	{
		          homeDir => $home_dir,
		        configDir => $config_dir,
		         jsonFile => $json_data_file,
		         cacheDir => $cache_font_dir,
		        targetDir => $target_dir,
		          tempDir => $temp_dir,
		installedVersions => $installed_versions,
	};
}

sub write_data {
	shift;
	my $local_data = shift;
	my $json_file  = shift;
	my $installed_versions = encode_json($local_data);
	_write_file($json_file, $installed_versions);
}

sub _read_file {
	my $entry = shift;
	open my $fh, "<:encoding(UTF-8)", $entry or die $!;
	local $/ = undef;
	my $content = <$fh>;
	close $fh;
	return $content;
}

sub _write_file {
	my $file = shift;
	my $json = shift;
	open my $fh, '>:encoding(UTF-8)', $file || die;
	print $fh $json;
	close $fh;
}

sub _reset_json_file {
	my $file = shift;
	my $info = kateb::FontInfo->new;
	my %hash = ();
	foreach my $font_name (keys %{$info})
	{
		$hash{$font_name} = '';
	}

	my $json = encode_json( \%hash );
	_write_file($file, $json);
}

1;
