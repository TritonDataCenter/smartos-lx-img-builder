use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub struct Opts {
    #[structopt(
        name = "tar",
        long = "tar",
        short = "t",
        help = "lx userland tar file",
        required = true
    )]
    pub tar: String,
    #[structopt(
        name = "kernel",
        long = "kernel",
        short = "k",
        help = "the kernel version",
        required = true
    )]
    pub kernel: String,
    #[structopt(
        name = "min_platform",
        long = "min",
        short = "m",
        help = "the minimum platform required for the image",
        required = true
    )]
    pub min_platform: String,
    #[structopt(
        name = "name",
        long = "name",
        short = "n",
        help = "the name of the image as it would appear in the manifest",
        required = true
    )]
    pub name: String,
    #[structopt(
        name = "description",
        long = "description",
        short = "d",
        help = "the description of the image as it would appear in the manifest",
        required = true
    )]
    pub description: String,
    #[structopt(
        name = "url",
        long = "url",
        short = "u",
        help = "the url to information about the image as it would appear in the manifest",
        required = true
    )]
    pub url: String,
    #[structopt(
        name = "zpool",
        long = "zpool",
        short = "z",
        help = "the zpool to use when creating our temporary image",
        required = true
    )]
    pub zpool: String,
}

pub fn get_opts() -> Opts {
    Opts::from_args()
}
