# کاتب

نصب کننده‌ی فونت‌های آزاد فارسی

‫کاتب یک نصب کننده و به روز رسان برای فونت‌های آزاد فارسی است.



[toc]



## نصب

- نصب برای اولین بار
- به روز رسانی از نسخه‌های قبل از ورژن۲(نسخه‌های پرل)

### نصب برای اولین بار

برای نصب کاتب، یکی از روش‌های زیر را انتخاب کنید:



- نصب برای کاربر فعلی (پیشنهاد اصلی)
  این روش برنامه را فقط برای حساب کاربری شما نصب می‌کند و فایل اجرایی در مسیر `~/.local/bin` قرار می‌گیرد:

```bash
curl -fsSL https://raw.githubusercontent.com/kiamazi/kateb/main/scripts/install.sh | bash
```

- نصب برای همه‌ی کاربران سیستم
  این روش برنامه را به‌صورت سراسری نصب می‌کند و فایل اجرایی در مسیر `/usr/local/bin` قرار می‌گیرد. اجرای این دستور به دسترسی مدیر سیستم نیاز دارد:

```bash
curl -fsSL https://raw.githubusercontent.com/kiamazi/kateb/main/install.sh | sudo bash
```

- نصب با کمک cargo

```bash
cargo install --git https://github.com/kiamazi/kateb
```

- نصب از منبع کد

```bash
git clone git@github.com:kiamazi/kateb.git
cd kateb
cargo build --release
```

- یا نسخه اجرایی را بر اساس سیستم‌عامل خودتان مستقیما از (صفحه انتشار)[https://github.com/kiamazi/kateb/releases] دانلود کنید، آن را به `kateb` تغییر نام دهید و در مسیری که PATH آن را پیدا کند، مثلا `~/.local/bin` کپی کنید


### به روز رسانی از نسخه‌های قبل از ورژن۲(نسخه‌های پرل)

برای به روزرسانی، ابتدا نسخه‌ی قدیمی را حذف کنید


- اگر کاتب را برای کاربر فعلی و بدون sudo نصب کرده‌اید

```bash
which kateb | xarg rm
```

یا

```bash
wich kateb
# ~/path/to/kateb

rm ~/path/to/kateb
```

و برای حذف ماژول پرل(اگر مایل به حذف کامل ماژول نیستید، این مرحله قابل چشم‌پشی است)

```bash
perl -M kateb -e 'print $INC{"kateb.pm"}' | xargs dirname | xargs -I {} rm -rf {}/kateb
```

- یا اگر برای تمام کاربران و با استفاده از sudo آن را نصب کرده‌اید

```bash
 which kateb | xarg sudo rm
```

یا

```bash
wich kateb
# ~/path/to/kateb

sudo rm ~/path/to/kateb
```

و برای حذف ماژول پرل(اگر مایل به حذف کامل ماژول نیستید، این مرحله قابل چشم‌پشی است)

```bash
perl -M kateb -e 'print $INC{"kateb.pm"}' | xargs dirname | xargs -I {} sudo rm -rf {}/kateb
```

---

به خاطر اینکه مسیر نصب فونت‌ها هم در نسخه جدید تغییر کرده، بهتر است پوشه‌ی قدیمی فونت را هم حذف کنید، البته از آنجایی که ممکن است همین حالا در حال استفاده از آن‌ها باشید و برای اینکه مشکلی در نمایش فونت‌ها در فاصله‌ی بین حذف پوشه قدیمی و نصب جدید پیش نیاید بهتر است ابتدا فونت‌ها را با استفاده از ورژن جدید نصب کنید و بعد از آن پوشه‌ی قدیمی را حذف کنید.

- کاتب را با یکی از روش‌های پیشنهاد شده نصب کنید، سپس فونت‌هایی که میخواهید را اضافه کنید

- مسیر نصب قدیمی را حذف کنید

```bash
#linux
rm -rf ~/.local/share/fonts/farsifreefont

#mac
rm ~/Library/Fonts/farsifreefont
```

یا اگر فونت‌ها را برای تمام سیستم نصب کرده‌اید:

```bash
#linux
sudo rm -rf /usr/share/fonts/farsifreefont

#mac
sudo rm /Library/Fonts/farsifreefont
```

## استفاده

```bash
kateb <command> [option]
```

### list

نمایش فونت‌های پشتیبانی شده توسط کاتب
```bash
kateb list
```

### install

نصب یک فونت جدید یا تمام فونت‌های پشتیبانی شده

```bash
kateb install [font name(s) or all]
    
kateb install vazir
    #or
kateb install vazir sahel estedad
    #or
kateb install all
```

### update

به روز رسانی فونت نصب شده

```bash
kateb update [font name(s) or all]

kateb update shabnam
    #or
kateb update shabnam mikhad lalezar sahel
    #or
kateb update all
```

### reinstall

دوباره نصب کردن فونت نصب شده

```bash
kateb reinstall [font name(s) or all]

kateb reinstall behdad
    #or
kateb reinstall behdad ganjnameh parastoo
    #or
kateb reinstall all
```

### uninstall

حذف کردن فونت نصب شده

```bash
kateb uninstall [font name(s) or all]

kateb uninstall ario
    #or
kateb uninstall nika arad ziracode
    #or
kateb uninstall all
```

### fonts

لیست نام و نسخه‌ی فونت‌های نصب شده توسط کاتب

```bash
kateb fonts
```

### info

خلاصه اطلاعات در مورد منتشر کننده فونت

```bash
kateb info Mikhak
    #or
kateb info all
```

### help

```
~$ kateb help

Usage: kateb <COMMAND>

Commands:
  install    Install a new font
  update     Update an installed font
  reinstall  Reinstall an already-installed font
  uninstall  Uninstall an already-installed font
  list       List all supported Farsi fonts
  fonts      Show the fonts that are currently installed
  info       Display brief information about a font's publisher
  version    Display the kateb version [alias: -v]
  help       Print this message or the help of the given subcommand(s)

Options:
  -h, --help  Print help
```

## مسیر نصب

در صورتی‌که فونت‌های پشتیبانی شده را قبلا در یکی از مسیرهایی که به عنوان مسیرهای معتبر فونت توسط سیستم‌عامل شما شناخته می‌شوند کپی کرده‌اید یا توسط نسخه‌های قبل از ورژن۲ کاتب آن‌ها را نصب کرده‌اید، برای جلوگیری از تداخل بین نسخه‌های مختلف فونت، آن فونت‌ها را حذف کنید

مسیرهایی مانند

```
~/.local/share/fonts/
~/.local/share/fonts/farsifreefont
/usr/share/fonts/
/usr/share/fonts/farsifreefont

~/Library/Fonts/
~/Library/Fonts/farsifreefont
/Library/Fonts/
/Library/Fonts/farsifreefont
```

- ‫مسیر نصب فونت‌ها توسط کاتب در پوشه‌ی خانه کاربر:

```bash
#linux
~/.local/share/fonts/farsi-freefont

#mac
~/Library/Fonts/farsi-freefont
```


- ‫مسیر نصب سیستمی فونت:
```bash
#linux
/usr/share/fonts/truetype/farsi-freefont

#mac
/Library/Fonts/farsi-freefont
```

## فونت‌های پشتیبانی شده

لیست فونت‌هایی که در حال حاضر پشتیبانی می‌شوند

    arad
    ario
    behdad
    estedad
    farbod
    gandom
    ganjnameh
    lalezar
    mikhak
    nahid
    nastaliq
    nika
    noon
    parastoo
    pfont
    rooyin
    sahel
    samim
    shabnam
    shahab
    tanha
    vazir
    vazircode
    ziracode
