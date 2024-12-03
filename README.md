## fakefilter

A simple program I wrote to practice data serialization/deserialization in Rust. It prints out a list of everyone who isn't following you back on Instagram.

In order to check your own followers, you need to [request your data](https://help.instagram.com/181231772500920?helpref=faq_content) from Instagram. You only *need* to request your **Followers and Following** under the **Connections** section. 
When you get your personal data, create a `data` directory in the same level as `src` and unzip it there. 

Make sure the folders are organized as `data/connections/followers_and_following`, otherwise the code will not work. 

To run the program, make sure you have `cargo` installed. Then run `cargo run` and the output should be printed. 
