#!/usr/bin/env perl

use 5.012;

use strict;
use warnings;

use lib '../lib';

use kateb;
use kateb::Install;
use kateb::LocalData;
my $version = $kateb::VERSION;


use Term::ANSIColor qw(:constants);

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

#------------------- prepare -------------------#
my $local_data = kateb::LocalData->new;
#use Data::Dumper;
#say Dumper $local_data;
#exit;
print "\n";

#----------------- Check ARGVs -----------------#
usage() unless @ARGV;

usage() if (@ARGV && $ARGV[0] !~ m/^install$|^reinstall$|^update$|^list$|^fonts$|^version$|\-v/);

if (@ARGV && $ARGV[0] =~ m/version|\-v/)
{
	print "version: $c{bgreen}". $version ."$c{reset}\n";
	exit 0;
}

if (@ARGV && $ARGV[0] =~ m/^list$/)
{
	my $num;
	say "installed fonts:\n";
	foreach my $font_name ( sort {lc $a cmp lc $b} keys %{$local_data->{installedVersions}})
	{
		if ($local_data->{installedVersions}->{$font_name})
		{
			printf("%20s : %s\n", $font_name, $local_data->{installedVersions}->{$font_name});
			++$num;
		}
	}
	say "No fonts have been installed by kateb yet\n" unless $num;
	print "\n";
	exit 0;
}

if (@ARGV && $ARGV[0] =~ m/^fonts$/)
{
	require kateb::FontInfo;
	my $info = kateb::FontInfo->new;
	my @fonts_list = keys %{$info};
	say "List of fonts supported by kateb\n";
	say "\t$_" foreach @fonts_list;
	print "\n";
}

if (@ARGV && $ARGV[0] =~ m/^install$/)
{
	shift @ARGV;
	my $install = kateb::Install->new(@ARGV);
	if ($install->{error})
	{
		say $install->{message};
		say "- $_" foreach @{$install->{errlist}};
		print "\n";
		say "To see the list of fonts supported by kateb, try:\n\n\tkateb fonts\n";

		exit $install->{error};
	}

	$install->install($local_data);

	exit 0;
}

if (@ARGV && $ARGV[0] =~ m/^update$/)
{
	shift @ARGV;
	my $install = kateb::Install->new(@ARGV);
	if ($install->{error})
	{
		say $install->{message};
		say "- $_" foreach @{$install->{errlist}};
		print "\n";
		say "To see the list of fonts supported by kateb, try:\n\n\tkateb fonts\n";
		
		exit $install->{error};
	}

	$install->update($local_data);

	exit 0;
}

if (@ARGV && $ARGV[0] =~ m/^reinstall$/)
{
	shift @ARGV;
	my $install = kateb::Install->new(@ARGV);
	if ($install->{error})
	{
		say $install->{message};
		say "- $_" foreach @{$install->{errlist}};
		print "\n";
		say "To see the list of fonts supported by kateb, try:\n\n\tkateb fonts\n";

		exit $install->{error};
	}

	$install->reinstall($local_data);

	exit 0;
}

sub usage {
	print <<END_USAGE;

kateb <command> [option]

commands:
  install
    install new font
  update
    update available font
  list
    list of installed fonts versions
  fonts
    show all farsi free fonts supported
  version | -v
    kateb version

options:
      -a | all
        install or update all fonts

END_USAGE
exit 1;
}