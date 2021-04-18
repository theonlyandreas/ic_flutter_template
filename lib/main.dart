import 'package:flutter/material.dart';
import 'package:ic_ffi/ic.dart';

void main() => runApp(MyApp());

class MyApp extends StatelessWidget {
  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      title: 'Flutter Demo',
      theme: ThemeData(
        primarySwatch: Colors.blue,
      ),
      home: MyHomePage(title: 'Flutter Demo Home Page'),
    );
  }
}

class MyHomePage extends StatefulWidget {
  MyHomePage({Key? key, required this.title}) : super(key: key);
  final String title;
  @override
  _MyHomePageState createState() => _MyHomePageState();
}

class _MyHomePageState extends State<MyHomePage> {
  int _counter = 0;
  IC ic = IC();

  @override
  void initState() {
    super.initState();
    IC.setup();
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        title: Text(widget.title),
      ),
      body: Center(
        child: Column(
          mainAxisAlignment: MainAxisAlignment.center,
          children: [
            Text(
              'You have pushed the button this many times:',
            ),
            Text(
              '$_counter',
              style: Theme.of(context).textTheme.headline4,
            ),
            const SizedBox(height: 100),
            RaisedButton(
              color: Colors.greenAccent,
              child: Text(
                'Scrape rust-lang.org',
                style: TextStyle(
                  color: Colors.white,
                ),
              ),
              onPressed: _showWebPage,
            )
          ],
        ),
      ),
    );
  }

  void _showWebPage() async {
    final html = await ic.loadPage('https://www.rust-lang.org/');
    showModalBottomSheet(
      context: context,
      isScrollControlled: true,
      isDismissible: true,
      builder: (context) => SingleChildScrollView(
        child: Text(html),
      ),
    );
  }
}
