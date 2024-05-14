// GENERATED CODE - DO NOT MODIFY BY HAND

// **************************************************************************
// AutoRouterGenerator
// **************************************************************************

// ignore_for_file: type=lint
// coverage:ignore-file

// ignore_for_file: no_leading_underscores_for_library_prefixes
import 'package:auto_route/auto_route.dart' as _i2;
import 'package:flutter/material.dart' as _i3;
import 'package:mobile_app/src/features/events/view/home_page.dart' as _i1;

abstract class $BeeLiveRouter extends _i2.RootStackRouter {
  $BeeLiveRouter({super.navigatorKey});

  @override
  final Map<String, _i2.PageFactory> pagesMap = {
    Home.name: (routeData) {
      final args = routeData.argsAs<HomeArgs>(orElse: () => const HomeArgs());
      return _i2.AutoRoutePage<dynamic>(
        routeData: routeData,
        child: _i1.HomePage(key: args.key),
      );
    }
  };
}

/// generated route for
/// [_i1.HomePage]
class Home extends _i2.PageRouteInfo<HomeArgs> {
  Home({
    _i3.Key? key,
    List<_i2.PageRouteInfo>? children,
  }) : super(
          Home.name,
          args: HomeArgs(key: key),
          initialChildren: children,
        );

  static const String name = 'Home';

  static const _i2.PageInfo<HomeArgs> page = _i2.PageInfo<HomeArgs>(name);
}

class HomeArgs {
  const HomeArgs({this.key});

  final _i3.Key? key;

  @override
  String toString() {
    return 'HomeArgs{key: $key}';
  }
}
