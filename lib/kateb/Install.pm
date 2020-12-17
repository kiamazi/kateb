package kateb::Install;
$kateb::Install::VERSION = '01.00.22';

use strict;
use warnings;
use 5.012;

use kateb::FontInfo;
use JSON::PP;
use LWP::UserAgent ();
use URI            ();
use HTTP::Date     ();
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
	my $class      = shift;
	my @args       = @_;
	my $self;
	if (not $args[0])
	{
		$self = {
			error   => 1,
			message => "type font name(s) or use all|-a option",
			errlist => [],
			fonts   => '',
		};
	} elsif ($args[0] =~ /^all$|^-a$/) {
		$self ={
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

	my $temp_dir   = $local_data->{tempDir};
	my $cache_dir  = $local_data->{cacheDir};
	my $target_dir = $local_data->{targetDir};
	my $json_file  = $local_data->{jsonFile};

	foreach my $font_name (@fonts)
	{
		my $version = _online_version($info->{$font_name}->{api});
		if (
			$local_data->{installedVersions}->{$font_name} and
			$version eq $local_data->{installedVersions}->{$font_name}
			)
		{
			print $c{bgreen} . $font_name . $c{reset};
			print ", The latest version is already installed: ";
			say $c{bgreen} . $version . $c{reset};
			next;
		}
		_do($font_name, $local_data, $version);
	}
	kateb::LocalData->write_data($local_data->{installedVersions}, $json_file);
}

sub update {
	my $self       = shift;
	my $local_data = shift;
	my @fonts      = @{$self->{fonts}};
	my $info       = kateb::FontInfo->new;

	my $temp_dir   = $local_data->{tempDir};
	my $cache_dir  = $local_data->{cacheDir};
	my $target_dir = $local_data->{targetDir};
	my $json_file  = $local_data->{jsonFile};

	foreach my $font_name (@fonts)
	{
		if (not $local_data->{installedVersions}->{$font_name})
		{
			print $c{bred} . $font_name . $c{reset};
			print " is not already installed, but can be installed with:\n\n";
			say $c{bblue} . "\tkateb install $font_name\n". $c{reset};
			next;
		}

		my $version = _online_version($info->{$font_name}->{api});

		if ($version eq $local_data->{installedVersions}->{$font_name})
		{
			print $c{bgreen} . $font_name . $c{reset};
			print ", The latest version is already installed: ";
			say $c{bgreen} . $version . $c{reset};
			next;
		}
		_do($font_name, $local_data, $version);
	}
	kateb::LocalData->write_data($local_data->{installedVersions}, $json_file);
}

sub reinstall {
	my $self       = shift;
	my $local_data = shift;
	my @fonts      = @{$self->{fonts}};
	my $info       = kateb::FontInfo->new;

	my $temp_dir   = $local_data->{tempDir};
	my $cache_dir  = $local_data->{cacheDir};
	my $target_dir = $local_data->{targetDir};
	my $json_file  = $local_data->{jsonFile};

	foreach my $font_name (@fonts)
	{
		if (not $local_data->{installedVersions}->{$font_name})
		{
			print $c{bred} . $font_name . $c{reset};
			print " is not already installed, but can be installed with:\n\n";
			say $c{bblue} . "\tkateb install $font_name\n". $c{reset};
			next;
		}
		my $version = _online_version($info->{$font_name}->{api});
		_do($font_name, $local_data, $version);
	}
	kateb::LocalData->write_data($local_data->{installedVersions}, $json_file);
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
	my $json_file  = $local_data->{jsonFile};

	my $info       = kateb::FontInfo->new;

	if ($font_name eq 'lalezar')
	{
		my $url = $info->$font_name($version);
		my $archive_file = catfile($cache_dir, "Lalezar-Regular.ttf");
		_download( $url, $archive_file );
		_copy_fonts( $target_dir, $archive_file );
		$local_data->{installedVersions}->{$font_name} = $version;
		say $c{bgreen} . $font_name . $c{reset} . " installed. version: " . $c{bgreen} . $version . $c{reset};
	} else
	{
		my $url = $info->$font_name($version);
		my $archive_file = catfile($temp_dir, "$font_name-$version.zip");

		_download( $url, $archive_file );

		my @extracted_fonts =
		_unzip($font_name, $archive_file, $cache_dir);

		$local_data->{installedVersions}->{$font_name} = $version;

		_copy_fonts($target_dir, @extracted_fonts) if @extracted_fonts;

		say $c{bgreen} . $font_name . $c{reset} . " installed. version: " . $c{bgreen} . $version . $c{reset};
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
	my $user_agent = LWP::UserAgent->new
	(
		ssl_opts => { verify_hostname => 1 },
		keep_alive => 1
	);
	my $json_api_response = $user_agent->get($api);
	my $tags = decode_json( $json_api_response->{_content} );
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
	} else
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

	$url = URI->new($url);

	my $ua = LWP::UserAgent->new(
		agent	  => "kateb",
		keep_alive => 1,
		env_proxy  => 1,
	);

	my $file;       # name of file we download into
	my $length;     # total number of bytes to download
	my $flength;    # formatted length
	my $size = 0;   # number of bytes received
	my $start_t;    # start time of download
	my $last_dur;   # time of last callback
	my $FILE_HANDLE;

	$shown = 0;     # have we called the show() function yet

	$SIG{INT} = sub { die "Interrupted\n"; };

	$| = 1;         # autoflush

	my $res = $ua->request(
		HTTP::Request->new(GET => $url),
		sub {
			unless (defined $file) {
				my $res = $_[1];

				$file = $archive_file;

				print "Saving to '$file'...\n";
				use Fcntl qw(O_WRONLY O_EXCL O_CREAT);
				sysopen($FILE_HANDLE, $file, O_WRONLY | O_EXCL | O_CREAT)
					|| die "Can't open $file: $!";

				unless (fileno($FILE_HANDLE)) {
					open($FILE_HANDLE, ">", $file) || die "Can't open $file: $!\n";
				}
				binmode $FILE_HANDLE;
				$length   = $res->content_length;
				$flength  = fbytes($length) if defined $length;
				$start_t  = time;
				$last_dur = 0;
			}

			print $FILE_HANDLE $_[0] or die "Can't write to $file: $!\n";
			$size += length($_[0]);

			if (defined $length) {
				my $dur = time - $start_t;
				if ($dur != $last_dur) {	# don't update too often
					$last_dur = $dur;
					my $perc = $size / $length;
					my $speed;
					$speed = fbytes($size / $dur) . "/sec" if $dur > 3;
					my $secs_left = fduration($dur / $perc - $dur);
					$perc = int($perc * 100);
					my $show = "$perc% of $flength";
					$show .= " (at $speed, $secs_left remaining)" if $speed;
					show($show, 1);
				}
			}
			else {
				show(fbytes($size) . " received");
			}
		}
	);

	if (fileno($FILE_HANDLE)) {
		close($FILE_HANDLE) || die "Cant write to $file: $!\n";

		show("");	# clear text
		print "\r";
		print fbytes($size);
		print " of ", fbytes($length) if defined($length) && $length != $size;
		print " received";
		my $dur = time - $start_t;
		if ($dur) {
			my $speed = fbytes($size / $dur) . "/sec";
			print " in ", fduration($dur), " ($speed)";
		}
		print "\n";

		if (my $mtime = $res->last_modified) {
			utime time, $mtime, $file;
		}

		if ($res->header("X-Died") || !$res->is_success) {
			if (my $died = $res->header("X-Died")) {
				print "$died\n";
			}
			if (-t) {
				print "Transfer aborted.  Delete $file? [n] ";
				my $ans = <STDIN>;
				if (defined($ans) && $ans =~ /^y\n/) {
					unlink($file) && print "Deleted.\n";
				}
				elsif ($length > $size) {
					print "Truncated file kept: ", fbytes($length - $size),
						" missing\n";
				}
				else {
					print "File kept.\n";
				}
				exit 1;
			}
			else {
				print "Transfer aborted, $file kept\n";
			}
		}
	}

	# Did not manage to create any file
#   print "\n" if $shown;
	if (my $xdied = $res->header("X-Died")) {
		print "kateb: Aborted\n$xdied\n";
	}
	else {
		print $c{bblue}, $url, " downloaded$c{reset}\n";
	}
}

sub fbytes {
	my $n = int(shift);
	if ($n >= 1024 * 1024) {
		return sprintf "%.3g MB", $n / (1024.0 * 1024);
	}
	elsif ($n >= 1024) {
		return sprintf "%.3g KB", $n / 1024.0;
	}
	else {
		return "$n bytes";
	}
}

sub fduration {
	use integer;
	my $secs = int(shift);
	my $hours = $secs / (60 * 60);
	$secs -= $hours * 60 * 60;
	my $mins = $secs / 60;
	$secs %= 60;
	if ($hours) {
		return "$hours hours $mins minutes";
	}
	elsif ($mins >= 2) {
		return "$mins minutes";
	}
	else {
		$secs += $mins * 60;
		return "$secs seconds";
	}
}


BEGIN {
	my @ani = qw(- \ | /);
	my $ani = 0;

	sub show {
		my ($mess, $show_ani) = @_;
		print "\r$mess" . (" " x (75 - length $mess));
		my $msg = $show_ani ? $ani[$ani++]. "\b" : ' ';
		print $msg;
		$ani %= @ani;
		$shown++;
	}
}



1;
